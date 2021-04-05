
extern crate xcb;

use std::sync::Arc;
use crate::{
	Stat, Data, WindowCommand, Event, DisplayEvent, KeyEvent,
	MouseEvent, event
};

const DEFAULT_WINDOW_DEPTH: u8 = 24;

pub struct Connection {
	connection: Arc<xcb::Connection>,
	preference: i32,
	visual: Option<xcb::Visualtype>,
	delete: Option<xcb::Atom>
}

impl Connection {
	pub fn open() -> Result<Self, Option<super::ConnectionError>> {
		let (connect, num) = match xcb::Connection::connect(None) {
			Err(_) => return Err(None),
			Ok((c, n)) => (c, n)
		};
		let setup = connect.get_setup();
		let screen = match setup.roots().nth(num as usize) {
			None => return Err(None),
			Some(screen) => screen
		};
		let depths = screen.allowed_depths();
		let mut depth = None;
		for d in depths {
			match d.depth() {
				DEFAULT_WINDOW_DEPTH => { depth = Some(d); },
				_ => ()
			}
		}
		let visuals = depth.map(|d| d.visuals());
		let mut visual = None;
		if let Some(visuals) = visuals {
			for v in visuals {
				if v.class() as u32 == xcb::VISUAL_CLASS_TRUE_COLOR {
					visual = Some(v);
					break;
				}
			}
		}
		let root = screen.root();

		let cookie = xcb::intern_atom(&connect, true, "WM_PROTOCOLS");
		cookie.get_reply();
		let cookie = xcb::intern_atom(&connect, false, "WM_DELETE_WINDOW");
		let delete = cookie.get_reply().ok().map(|r| r.atom());
		
		Ok(Self {
			connection: Arc::new(connect),
			preference: num,
			visual,
			delete
		})
	}
	
	pub fn flush(&self) {
		self.connection.flush();
	}
	
	pub fn create_window(&self) -> Window {
		let setup = self.connection.get_setup();
		let screen = setup.roots().nth(self.preference as usize).unwrap();
		let window = window(&self.connection, &screen);
		Window::new(window, self.connection.clone(), self.visual, self.delete)
	}
}

pub struct Window {
	window: xcb::Window,
	connection: Arc<xcb::Connection>,
	visual: Option<xcb::Visualtype>,
	delete: Option<xcb::Atom>
}

impl Window {
	fn new(window: xcb::Window, connection: Arc<xcb::Connection>,
		visual: Option<xcb::Visualtype>, delete: Option<xcb::Atom>) -> Self {
		Self {
			window,
			connection,
			visual,
			delete
		}
	}
}

impl Window {

	pub fn id(&self) -> u32 {
		self.connection.generate_id()
	}

	fn property<T>(&self, mode: xcb::PropMode, prop: xcb::AtomEnum, ty: xcb::AtomEnum, data: &[T]) {
		xcb::change_property(
			&self.connection,
			mode as u8,
			self.window,
			prop,
			ty,
			8,
			data
		);
	}

	fn configure(&self, values: &[(u16, u32)]) {
		xcb::configure_window(
			&self.connection,
			self.window,
			values
		);
	}

	fn geometry(&self) -> Option<xcb::GetGeometryReply> {
		xcb::get_geometry(&self.connection, self.window).get_reply().ok()
	}

	fn stat_position(&self) -> Option<(u32, u32)> {
		self.geometry().map(|g| (g.x() as u32, g.y() as u32))
	}

	fn stat_dimension(&self) -> Option<(u32, u32)> {
		self.geometry().map(|g| (g.width() as u32, g.height() as u32))
	}

	fn stat_depth(&self) -> Option<u8> {
		self.geometry().map(|g| g.depth())
	}

	fn title(&self, name: &str) {
		use xcb::*;
		self.property(PROP_MODE_REPLACE, ATOM_WM_NAME, ATOM_STRING, name.as_bytes());
	}

	fn icon_title(&self, name: &str) {
		use xcb::*;
		self.property(PROP_MODE_REPLACE, ATOM_WM_ICON_NAME, ATOM_STRING, name.as_bytes());
	}

	fn x(&self, x: u32) {
		self.configure(&[(
			xcb::CONFIG_WINDOW_X as u16, x
		)]);
	}

	fn y(&self, y: u32) {
		self.configure(&[(
			xcb::CONFIG_WINDOW_Y as u16, y
		)]);
	}

	fn move_to(&self, x: u32, y: u32) {
		self.configure(&[
			(xcb::CONFIG_WINDOW_X as u16, x),
			(xcb::CONFIG_WINDOW_Y as u16, y)
		]);
	}

	fn width(&self, width: u32) {
		self.configure(&[(
			xcb::CONFIG_WINDOW_WIDTH as u16, width
		)]);
	}

	fn height(&self, height: u32) {
		self.configure(&[(
			xcb::CONFIG_WINDOW_HEIGHT as u16, height
		)]);
	}

	fn resize(&self, width: u32, height: u32) {
		self.configure(&[
			(xcb::CONFIG_WINDOW_WIDTH  as u16, width),
			(xcb::CONFIG_WINDOW_HEIGHT as u16, height)
		]);
	}

	fn map(&self) {
		xcb::map_window(&self.connection, self.window);
	}

	fn unmap(&self) {
		xcb::unmap_window(&self.connection, self.window);
	}

	fn stack_above(&self) {
		self.configure(&[(
			xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE
		)]);
	}

	fn stack_below(&self) {
		self.configure(&[(
			xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_BELOW
		)]);
	}

	fn window_event_map(&self, event: Option<xcb::base::GenericEvent>) -> Option<Event> {
		if event.is_none() {
			if let Err(_) = self.connection.has_error() {
				return Some(Event::Terminate);
			}
		}

		event.map(|e| {
			let response = event_type(&e);

			match response {
				xcb::EXPOSE => {
					DisplayEvent::Expose(event::xcb::expose(&e)).into()
				},

				xcb::KEY_PRESS => {
					KeyEvent::Press(event::xcb::key_press(&e)).into()
				},

				xcb::KEY_RELEASE => {
					KeyEvent::Release(event::xcb::key_release(&e)).into()
				},

				xcb::BUTTON_PRESS => {
					MouseEvent::Press(event::xcb::button_press(&e)).into()
				},

				xcb::BUTTON_RELEASE => {
					MouseEvent::Release(event::xcb::button_release(&e)).into()
				},

				xcb::MOTION_NOTIFY => {
					MouseEvent::Move(event::xcb::mouse_move(&e)).into()
				},

				xcb::ENTER_NOTIFY => {
					MouseEvent::Enter(event::xcb::mouse_enter(&e)).into()
				},

				xcb::LEAVE_NOTIFY => {
					MouseEvent::Leave(event::xcb::mouse_leave(&e)).into()
				},

				xcb::FOCUS_IN => DisplayEvent::FocusIn.into(),

				xcb::FOCUS_OUT => DisplayEvent::FocusOut.into(),

				xcb::CLIENT_MESSAGE => {
					let event = unsafe { xcb::cast_event::<xcb::ClientMessageEvent>(&e) };
					if event.type_() == 32 {
						if let Some(del) = self.delete {
							if del == event.data().data32()[0] {
								return Event::Terminate;
							}
						}
					}
					Event::Unknown(Some(response.into()))
				},

				_ => Event::Unknown(Some(response.into()))
			}
		})
	}

	fn clear(&self) {
		xcb::clear_area(&self.connection, true, self.window, 0, 0, 0, 0);
	}
}

impl super::WindowContext for Window {
	fn event(&self) -> Option<Event>
	{
		let event = self.connection.wait_for_event();
		self.window_event_map(event)
	}

	fn poll(&self) -> Option<Event>
	{
		let event = self.connection.poll_for_event();
		self.window_event_map(event)
	}

	fn stat(&self, status: Stat) -> Option<Data>
	{
		use crate::{stat::{WindowStat, XcbStat}, data::{WindowData, XcbData}};

		match status {
			Stat::Window(status) => {
				Some((match status {
					WindowStat::Position => WindowData::Position(self.stat_position()?),
					WindowStat::Dimension => WindowData::Dimension(self.stat_dimension()?),
					WindowStat::Depth => WindowData::Depth(self.stat_depth()?)
				}).into())
			},
			Stat::Xcb(status) => {
				Some((match status {
					XcbStat::Connection => XcbData::Connection(self.connection.get_raw_conn()),
					XcbStat::Window => XcbData::Window(self.window),
					XcbStat::VisualType => XcbData::VisualType(self.visual)
				}).into())
			},
			_ => None
		}
	}

	fn window(&self, command: &WindowCommand)
	{
		use WindowCommand::*;
		match command {
			Title(name) => self.title(name),
			Dimension((w, h)) => self.resize(*w, *h),
			Origin((x, y)) => self.move_to(*x, *y),
			Map => self.map(),
			Unmap => self.unmap(),
			StackAbove => self.stack_above(),
			StackBelow => self.stack_below(),
			Clear => self.clear(),
			Update => { self.connection.flush(); }
		}
	}
	
	fn update(&self) {
		self.connection.flush();
	}
}

impl Drop for Window {
	fn drop(&mut self) {
		xcb::destroy_window(&self.connection, self.window);
	}
}

#[allow(unused_parens)]
static EVENT_MASK: xcb::EventMask = (
	xcb::EVENT_MASK_EXPOSURE |
	xcb::EVENT_MASK_KEY_PRESS |
	xcb::EVENT_MASK_BUTTON_PRESS |
	xcb::EVENT_MASK_BUTTON_RELEASE |
	xcb::EVENT_MASK_POINTER_MOTION |
	xcb::EVENT_MASK_BUTTON_MOTION |
	xcb::EVENT_MASK_BUTTON_1_MOTION |
	xcb::EVENT_MASK_BUTTON_2_MOTION |
	xcb::EVENT_MASK_BUTTON_3_MOTION |
	xcb::EVENT_MASK_BUTTON_4_MOTION |
	xcb::EVENT_MASK_BUTTON_5_MOTION |
	xcb::EVENT_MASK_ENTER_WINDOW |
	xcb::EVENT_MASK_LEAVE_WINDOW |
	xcb::EVENT_MASK_FOCUS_CHANGE |
	/*xcb::EVENT_MASK_RESIZE_REDIRECT |*/
	xcb::EVENT_MASK_STRUCTURE_NOTIFY |
	xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY |
	/*xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT |*/
	xcb::EVENT_MASK_VISIBILITY_CHANGE
);

#[inline]
fn event_type(e: &xcb::base::GenericEvent) -> u8
{
	e.response_type() & !0x080
}

fn window(conn: &xcb::Connection, screen: &xcb::Screen) -> u32
{
	let id = conn.generate_id();

	let values = [
		(xcb::CW_EVENT_MASK, EVENT_MASK)
	];

	let (x, y) = (0, 0);
	let (width, height) = (1, 1);
	let border = 10;

	xcb::create_window(
		conn,
		xcb::COPY_FROM_PARENT as u8,
		id,
		screen.root(),
		x as i16,
		y as i16,
		width as u16,
		height as u16,
		border,
		xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
		screen.root_visual(),
		&values
	);
	id
}

#[inline]
fn gc(conn: &xcb::Connection, window: xcb::Window, color: u32) -> u32
{
	let id = conn.generate_id();
	xcb::create_gc(
		conn, id, window, &[
			(xcb::GC_FOREGROUND,
			 color),
			(xcb::GC_GRAPHICS_EXPOSURES,
			 0)
		]
	);
	id
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn xcb_connection() {
		let connect = Connection::open().unwrap();
		let window = connect.create_window();
		window.resize(150, 150);
		window.map();
		connect.flush();
		let timeout = std::time::Duration::from_millis(5000);
		std::thread::sleep(timeout);
	}
}

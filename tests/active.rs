extern crate ren;

#[test]
fn main()
{
    // Open a connection
    let mut connect = ren::Connection::new();
    let token = connect.begin();

    // Assert that it is active
    assert_eq!(true, connect.active(&token));

    // End the connection
    connect.end(&token);
    
    // Assert that it is not active
    assert_eq!(false, connect.active(&token));
}

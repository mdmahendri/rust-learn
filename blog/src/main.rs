use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("now test 123");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("now test 123", post.content());
}

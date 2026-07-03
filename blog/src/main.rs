use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("now test 123");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("now test 123", post.content());
}

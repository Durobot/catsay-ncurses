//use cursive::Cursive;
use cursive::views::TextView;
use cursive::event::Key;

fn main()
{
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();
    let cat_text = "Meow!
 \\
  \\
   /\\_/\\
  ( o o )
  =( I )=";
    // Declaring the app layout
    siv.add_layer(TextView::new(cat_text));
    // Listen to Key::Esc and quit
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.run(); // starting the event loop
}

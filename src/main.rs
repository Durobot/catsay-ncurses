use cursive::Cursive;
use cursive::traits::Identifiable; // for .with_id() and .call_on_id()
use cursive::views::{Checkbox, Dialog, EditView, ListView, TextView};
use cursive::event::Key;

struct CatsayOptions<'a>
{
    message: &'a str,
    dead: bool,
}

fn main()
{
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();
    create_form(&mut siv);
    // Listen to Key::Esc and quit
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.run(); // starting the event loop
}

fn create_form(siv: &mut Cursive) // this fn is called input_step() in the original project
{
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the cat")
            .content(
                ListView::new()
                    // warning: use of deprecated associated function `cursive::view::Nameable::with_id`:
                    // `with_id` is being renamed to `with_name`
                    .child("Message:", EditView::new().with_name("message")) //.with_id("message"))
                    .child("Dead?", Checkbox::new().with_name("dead")) //.with_id("dead")),
             )
            .button("OK",
                |s|
                {
                    // warning: use of deprecated associated function `cursive::Cursive::call_on_id`:
                    // `call_on_id` is being renamed to `call_on_name`
                    //let message = s.call_on_id("message", |t: &mut EditView| t.get_content()).unwrap();
                    //let is_dead = s.call_on_id("dead", |t: &mut Checkbox| t.is_checked()).unwrap();
                    let message = s.call_on_name("message", |t: &mut EditView| t.get_content()).unwrap();
                    let is_dead = s.call_on_name("dead", |t: &mut Checkbox| t.is_checked()).unwrap();
                    let options = CatsayOptions
                    {
                        message: &message,
                        dead: is_dead,
                    };
                    on_ok_button(s, &options);
                }),
    );
}

fn on_ok_button(siv: &mut Cursive, options: &CatsayOptions) // this fn is called result_step() in the original project
{
    let eye = if options.dead { "x" } else { "o" };
    let cat_text = format!("{msg}
 \\
  \\
   /\\_/\\
  ( {eye} {eye} )
  =( I )=",
        msg = options.message,
        eye = eye
    );
    // This “pops” the existing layer (i.e., the form layer) from the layers stack
    siv.pop_layer();
    siv.add_layer(
        Dialog::around(TextView::new(cat_text))
            .title("The cat says...")
            .button("OK", |s| s.quit()),
    );
}

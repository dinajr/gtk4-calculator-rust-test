use gtk4::{
//	gdk, gio,
	glib::{self, clone},
	prelude::*,
};
use meval::eval_str;

pub fn build_ui(application: &gtk4::Application) {
	let ui_src = include_str!("../layout.xml");
	let builder = gtk4::Builder::from_string(ui_src);

	let window = builder
	.object::<gtk4::ApplicationWindow>("window")
	.expect("Window Failed");

	let buffertext = gtk4::TextBuffer::builder()
	.build();

	let textview = builder
	.object::<gtk4::TextView>("text_view")
	.expect("text view failed");

	textview.set_buffer(Some(&buffertext));

	let bufferresult = gtk4::TextBuffer::builder()
	.build();

	let resultview = builder
	.object::<gtk4::TextView>("result_view")
	.expect("text view failed");

	resultview.set_buffer(Some(&bufferresult));


	let button7 = builder
	.object::<gtk4::Button>("button7")
	.expect("button7 failed");

	button7.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("7");
		}
	));

	let button8 = builder
	.object::<gtk4::Button>("button8")
	.expect("button8 failed");

	button8.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("8");
		}
	));

	let button9 = builder
	.object::<gtk4::Button>("button9")
	.expect("button9 failed");

	button9.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("9");
		}
	));

	let button4 = builder
	.object::<gtk4::Button>("button4")
	.expect("button4 failed");

	button4.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("4");
		}
	));

	let button5 = builder
	.object::<gtk4::Button>("button5")
	.expect("button5 failed");

	button5.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("5");
		}
	));

	let button6 = builder
	.object::<gtk4::Button>("button6")
	.expect("button6 failed");

	button6.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("6");
		}
	));

	let button1 = builder
	.object::<gtk4::Button>("button1")
	.expect("button1 failed");

	button1.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("1");
		}
	));

	let button2 = builder
	.object::<gtk4::Button>("button2")
	.expect("button2 failed");

	button2.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("2");
		}
	));

	let button3 = builder
	.object::<gtk4::Button>("button3")
	.expect("button3 failed");

	button3.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("3");
		}
	));

	let buttonminus = builder
	.object::<gtk4::Button>("buttonminus")
	.expect("buttonminus failed");

	buttonminus.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("-");
		}
	));


	let buttonplus = builder
	.object::<gtk4::Button>("buttonplus")
	.expect("buttonplus failed");

	buttonplus.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("+");
		}
	));


	let buttonmultiply = builder
	.object::<gtk4::Button>("buttonmultiply")
	.expect("buttonmultiply failed");

	buttonmultiply.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("*");
		}
	));


	let buttondivide = builder
	.object::<gtk4::Button>("buttondivide")
	.expect("buttondivide failed");

	buttondivide.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("/");
		}
	));

	let buttondot = builder
	.object::<gtk4::Button>("buttondot")
	.expect("buttondot failed");

	buttondot.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor(".");
		}
	));

	let button0 = builder
	.object::<gtk4::Button>("button0")
	.expect("button0 failed");

	button0.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.insert_at_cursor("0");
		}
	));

	let buttonerase = builder
	.object::<gtk4::Button>("buttonerase")
	.expect("buttonerase failed");

	buttonerase.connect_clicked(clone!(
		#[weak]
		buffertext,
		move |_| {
			buffertext.backspace(&mut buffertext.iter_at_offset(buffertext.char_count() * 4), true, true);
		}
	));


	let buttonenter = builder
	.object::<gtk4::Button>("buttonenter")
	.expect("buttonenter failed");

	buttonenter.connect_clicked(clone!(
		#[weak]
		buffertext,
		#[weak]
		bufferresult,
		move |_| {
			let text_expression: String = format!("{}", buffertext.text(&buffertext.iter_at_offset(0), &buffertext.iter_at_offset(buffertext.char_count() * 4), false));
			if let Ok(result) = eval_str(&text_expression) {
					bufferresult.set_text(&result.to_string().as_str());
			} else {
					bufferresult.set_text("Error");
			};
		}
	));


	window.set_application(Some(application));

	window.present();
}



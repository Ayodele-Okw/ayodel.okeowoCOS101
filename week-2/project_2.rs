fn main() {
	let toshibaqty:f64 = 2.0;
	let toshibaamount:f64 = 450000.00;
	println!("toshibaqty is {} and toshibaamount is {}",toshibaqty,toshibaamount);

	let macqty:f64 = 1.0;
	let macamount:f64 = 1500000.00;
	println!("macqty is {} and macamount is {}", macqty,macamount);

	let hpqty:f64 = 3.0;
	let hpamount:f64 = 750000.00;
	println!("hpqty is {} and hpamount is {}", hpqty,hpamount);

	let dellqty:f64 = 3.0;
	let dellamount:f64 = 2850000.00;
	println!("dellqty is {} and dellamount is {}", dellqty,dellamount);

	let acerqty:f64 = 1.0;
	let aceramount:f64 = 250000.00;
	println!("acerqty is {} and aceramount is {}", acerqty,aceramount);

	// sum
	let s = toshibaamount + macamount + hpamount + dellamount + aceramount;
	println!("sum is {}", s);

	// average
	let a = s / 5 as f64;
	println!("average is {}", a);



}
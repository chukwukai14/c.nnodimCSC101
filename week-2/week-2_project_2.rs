fn main () {
	let amount1 = 450_000.0;
	let qty1 = 2.0;

	let amount2 = 1_500_000.0;
	let qty2 = 1.0;

	let amount3 = 750_000.0;
	let qty3 = 3.0;

	let amount4 = 2_850_000.0;
	let qty4 = 3.0;

	let amount5 = 250_000.0;
	let qty5 = 1.0;

	let total_sum = (amount1 * qty1 as f64)
				  + (amount2 * qty2 as f64)
				  + (amount3 * qty3 as f64)
				  + (amount4 * qty4 as f64)
				  + (amount5 * qty5 as f64);

    let total_items = qty1 + qty2 + qty3 + qty4 + qty5;
    let average = total_sum / total_items as f64;

    println!("the total sum of the sales amounts is: N{:.2}",total_sum);
    println!("the average of the sales amount is: N{:.2}",average);
}
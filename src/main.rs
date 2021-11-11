/* practice09_progress bar by rust(cargo)
 * 		written by Matsumoto Kazuki
 *
 * topic
 * プログレスバーの作り方・gitへの依存の作り方
 *
 */

extern crate indicatif;

use std::cmp::min;
use std::thread;
use std::time::Duration;
use indicatif::*;


fn main ()
{
// ダウンロードプログレスバーサンプル
	println!("downloder sample");
	let mut downloaded = 0;
	let total_size = 231231231;
	let pb = ProgressBar :: new (total_size);


	pb.set_style (ProgressStyle :: default_bar ()
		.template ( "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})" )
		.progress_chars ( "=>-" ));

	while downloaded < total_size
	{
		let new = min (downloaded + 223211 , total_size);
		downloaded = new;

		pb.set_position (new);
		thread :: sleep (Duration :: from_millis ( 12 ));
	}

	pb.finish_with_message ( "downloaded" );


// 最適化経過時間サンプル
	println!("optimization sample");
	let mut ended = 0;
	let total_runs = 100;
	let pb = ProgressBar :: new (total_runs);


	pb.set_style (ProgressStyle :: default_bar ()
		.template ( "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}runs/{len}runs ({eta})" )
		.progress_chars ( "=>-" ));

	while ended < total_runs
	{
		let new = min (ended + 1 , total_runs);
		ended = new;

		pb.set_position (new);
		thread :: sleep (Duration :: from_millis ( 45 ));
	}

	pb.finish_with_message ( "optimized finish!" );

}
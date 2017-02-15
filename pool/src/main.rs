extern crate scoped_pool;

use scoped_pool::Pool;

fn main() {
    let pool = Pool::new(4);

    let mut buf = [0, 0, 0, 0];

    pool.scoped(|scope| {
        for i in &mut buf {
            scope.execute(move || *i += 1);
        }
    });

    println!("{:?}", buf);

    assert_eq!(&buf, &[1, 1, 1, 1]);
}

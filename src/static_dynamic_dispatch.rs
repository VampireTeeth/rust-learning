
#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;


trait F {
    fn f(&self);
}

impl F for A {
    fn f(&self) {
        println!("{:?}", self)
    }
}

impl F for B {
    fn f(&self) {
        println!("{:?}", self)
    }
}

fn static_dispatch<T: F>(t: &T) {
    t.f();
}

fn dynamic_dispatch(t: &dyn F) {
    t.f();
}

pub fn showcase1() {
    let a = A;
    let b = B;
    static_dispatch(&a);
    static_dispatch(&b);
    let input = "B";
    let o: &dyn F = match input {
        "A" => &A,
        "B" => &B,
        _ => panic!(),
    };
    dynamic_dispatch(o);
}


fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

fn strlen2<S>(s: S) -> usize
where
    S: AsRef<str> {
    s.as_ref().len()
}

pub fn showcase2() {
    println!("strlen(&'static str): {}", strlen("Hello world!"));
    println!("strlen(String): {}", strlen(String::from("Echo world!")));

    println!("strlen2(&'static str): {}", strlen2("Hello world!"));
    println!("strlen2(String): {}", strlen2(String::from("Echo world!")));
}

trait Hei {
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei: {}", self);
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei: {}", self);
    }
}

fn foo(h: &dyn Hei) {
    h.hei();
}

// fn bar(hs: &[dyn Hei]) {
//     for h in hs {
//         h.hei();
//     }
// }

pub fn showcase3() {
    foo(&"Yeheh");
    foo(&String::from("Hehea"));
}
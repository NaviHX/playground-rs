use std::rc::Rc;

pub type ChurchImpl<T> = Rc<dyn Fn(Rc<dyn Fn(T) -> T>) -> Rc<dyn Fn(T) -> T>>;

pub fn zero<T>() -> ChurchImpl<T> {
    Rc::new(|_| Rc::new(|x| x))
}

pub fn one<T: 'static>() -> ChurchImpl<T> {
    Rc::new(|f| Rc::new(move |x| f(x)))
}

pub fn incr<T: 'static>(church: ChurchImpl<T>) -> ChurchImpl<T> {
    Rc::new(move |f| {
        let n = church.clone();
        Rc::new(move |x| {
            let ff = f.clone();
            let n = n.clone();
            f(n(ff)(x))
        })
    })
}

pub fn add<T: 'static>(a: ChurchImpl<T>, b: ChurchImpl<T>) -> ChurchImpl<T> {
    Rc::new(move |f| {
        let (a, b) = (a.clone(), b.clone());
        Rc::new(move |x| {
            let f = f.clone();
            a(f.clone())(b(f.clone())(x))
        })
    })
}

pub fn instantialize_church<T>(church: ChurchImpl<T>, f: Rc<dyn Fn(T) -> T>, x: T) -> T {
    church(f)(x)
}

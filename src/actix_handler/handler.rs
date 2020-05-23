use super::extract::*;
use super::{Request, Response};

pub trait Handler<P: FromRequest> {
    fn call(&self, p: P) -> Response;
}

impl<T> Handler<()> for T
where
    T: Fn() -> Response,
{
    fn call(&self, _p: ()) -> Response {
        self()
    }
}

macro_rules! impl_handler {
    ($($T:ident),*) => {
        impl<$($T: FromRequest),*, __F>  Handler<($($T,)*)>  for __F where __F: Fn($($T),*) -> Response {
            #[allow(non_snake_case)]
            fn call(&self, req: ($($T,)*) ) -> Response {
                let ( $($T,)* ) = req;
                (self)($($T),*)
            }
        }
    }
}

impl_handler!(A);
impl_handler!(A, B);
impl_handler!(A, B, C);
impl_handler!(A, B, C, D);
impl_handler!(A, B, C, D, E);
impl_handler!(A, B, C, D, E, F);
impl_handler!(A, B, C, D, E, F, G);
impl_handler!(A, B, C, D, E, F, G, H);

pub fn handle<P: FromRequest, F: Handler<P>>(req: Request, f: F) -> Response {
    let p = P::from_req(&req);
    f.call(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f0() -> Response {
        Response
    }
    fn f1(_: Path) -> Response {
        Response
    }
    fn f2(_: Path, _: Data) -> Response {
        Response
    }
    fn f3(_: Path, _: Data, _: Query) -> Response {
        Response
    }
    fn f4(_: Path, _: Data, _: Query, _: State) -> Response {
        Response
    }

    #[test]
    fn test_handle() {
        let req = Request;
        handle(req, f0);
        handle(req, f1);
        handle(req, f2);
        handle(req, f3);
        handle(req, f4);
    }
}

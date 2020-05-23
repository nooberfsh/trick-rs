use super::Request;

pub trait FromRequest {
    fn from_req(req: &Request) -> Self;
}

impl FromRequest for Request {
    fn from_req(req: &Request) -> Self {
        req.clone()
    }
}

impl FromRequest for () {
    fn from_req(_req: &Request) -> Self {
        ()
    }
}

macro_rules! impl_req {
    ($name:ident) => {
        pub struct $name;
        impl FromRequest for $name {
            fn from_req(_req: &Request) -> Self {
                $name
            }
        }
    };
}

impl_req!(Path);
impl_req!(Query);
impl_req!(State);
impl_req!(Data);

macro_rules! impl_for_tuple {
    ($($T: ident),*) => {

        impl<$($T: FromRequest),*> FromRequest for ( $($T,)* ) {
            fn from_req(req: &Request) -> Self {
                ( $( <$T as FromRequest>::from_req(req) ,)* )
            }
        }
    }
}

impl_for_tuple!(A);
impl_for_tuple!(A, B);
impl_for_tuple!(A, B, C);
impl_for_tuple!(A, B, C, D);
impl_for_tuple!(A, B, C, D, E);
impl_for_tuple!(A, B, C, D, E, F);
impl_for_tuple!(A, B, C, D, E, F, G);
impl_for_tuple!(A, B, C, D, E, F, G, H);

#[cfg(test)]
mod tests {
    use super::*;

    fn t<T: FromRequest>(_t: T) {}

    #[test]
    fn test_single() {
        t(Path);
        t(Query);
        t(State);
        t(Data);
        t(Request);
        t(());
    }

    #[test]
    fn test_tuple() {
        t((Path,));
        t((Path, Query));
        t((Path, Query, State));
        t((Path, Query, State, Data));
        t((Path, Query, State, Data, Path));
        t((Path, Query, State, Data, Path, Query));
        t((Path, Query, State, Data, Path, Query, State));
        t((Path, Query, State, Data, Path, Query, State, Data));
    }
}

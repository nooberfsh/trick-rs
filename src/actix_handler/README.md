# actix-web handler

`actix_web::Rout` 有个 [to](https://docs.rs/actix-web/3.0.0-alpha.3/actix_web/struct.Route.html#method.to) , 它接受一个闭包， 神奇的地方是这个闭包可以有不一样的参数类型， 更神奇的是闭包可以有不同数量的参数。简化后的模型如下：
```rust
fn f1(p: Path) -> Response {...}
fn f2(p: Path, q: Query) -> Response {...} 
fn f3(p: Path, q: Query, l: Payload) -> Response {...}

fn handle<F: ?>(req: &Request, f: F) {...}

fn main() {
    let req = &Request;
    handle(req, f1);
    handle(req, f2);
    handle(req, f3);
}
```
这里 `F` 的 bound 是关键， 下面我们会开始一步步的实现这个 bound.

首先我们可以定义一个 `Handler` trait 用来抽象所有的 `handler` 函数 (比如上面的 f1, f2, f3)
```rust
pub trait Handler<P> {
    fn call(&self, p: P) -> Response;
}
```

然后用 `macro` 给每个满足要求的函数自动实现这个 trait， 举个具体的例子， 对于有3个参数的函数， 将会生成如下代码：
```rust
impl<P1, P2, P3, F> Handler<(P1, P2, P3)> for F where F: Fn(P1, P2, P3) -> Response {
    fn call(&self, p: (P1, P2, P3)) -> Response {
	self(p.0, p.1, p.2)
    }
}
```
这里的关键是把多个参数 `P1, P2, P3` 转化成了一个 `(P1, P2, P3)` 。现在我们 `handle` 函数变成下面这样：
```rust
fn handle<P, F: Handler<P>>(req: &Request, f: F) {...}
```

现在我们只需要`&Request` 获取 `Handler` 的参数

这里有一个细节： 并不是所有的函数都能传递给 `handle` 函数， 必须满足的条件是： 每个函数的所有参数能都从 `Request` 中获取: `Request -> Param`， 我们可以定义一些基本的能够使用的参数类型: `Path, Query, Payload ...` ， 并且定义如下 trait:
```rust
pub trait FromRequest {
    fn from_req(req: &Request) -> Self;
}

impl FromRequest for Path {...}
impl FromRequest for Query {...}
impl FromRequest for Payload {...}
```

现在我们的 `handle` 变成了这样：
```rust
fn handle<P： FromRequest, F: Handler<P>>(req: &Request, f: F) {...}
```
现在有个问题： `F` 能够接受 `(Path ,Query)` 类似这样的参数， 但是这个类型没有实现 `FromRequest`， 那怎么办？ 老方法， 还是用 `macro`， 实现细节可以看代码。

最后我们的 `handle` 长这样子：
```rust
fn handle<P： FromRequest, F: Handler<P>>(req: &Request, f: F) {
    let p = P::from_req(req);
    f.call(p)
}
```

We encountered a problem with the inference of types with sqlx.
We defined a custom enum for an array column and sqlx does not seem to like that.

# How to reproduce

When compiling this project, you have the following compilation error:

```
error[E0277]: the trait bound `Vec<Occupation>: Type<_>` is not satisfied
   --> src/main.rs:16:15
    |
16  |     let res = sqlx::query_as::<_, Speaker>("select id, name, occupation from speaker")
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Type<_>` is not implemented for `Vec<Occupation>`
    | 
   ::: /Users/test/.cargo/registry/src/github.com-1ecc6299db9ec823/sqlx-core-0.4.2/src/query_as.rs:160:8
    |
160 |     O: for<'r> FromRow<'r, DB::Row>,
    |        ---------------------------- required by this bound in `sqlx::query_as`
    |
    = help: the following implementations were found:
              <Vec<&[u8]> as Type<Postgres>>
              <Vec<&str> as Type<Postgres>>
              <Vec<(T1, T2)> as Type<Postgres>>
              <Vec<(T1, T2, T3)> as Type<Postgres>>
            and 22 others
    = note: required because of the requirements on the impl of `for<'r> FromRow<'r, _>` for `Speaker`
```

The compiler says `the trait Type<_> is not implemented for Vec<Occupation>` although `Vec<T: Type>` is [documented as a correct array type](https://docs.rs/sqlx/0.5.1/sqlx/postgres/types/index.html#arrays).

## Versions affected

We are using sqlx 0.4 to be compatible with current stable actix_web runtime.
It does not work with sqlx 0.5 neither.

## Workaround

We have found a workaround: use `Json<Vec<Occupation>>` as a type and change the request accordingly.

```
select name, to_jsonb(occupation) from speaker
```

## How do I run this?

```bash
docker run -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 sqlx-postgres 
cargo run
```

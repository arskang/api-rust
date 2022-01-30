# Reto CRUD básico con Rust

#### Librerías y otros

- [Cargo](https://crates.io/)
- [Actix](https://actix.rs/)

#### Iniciar el servidor localmente

`cargo run`

#### Endpoints

```
#[get("/version")]
#[get("/user")]
#[get("/user/{id}")]
#[post("/user")]
#[put("/user")]
#[delete("/user/{id}")]
```

#### Schemas

```json
// user
{
    "id": "(uuid)",
    "name": "(string)",
    "email": "(string)",
}
```

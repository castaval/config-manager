# Config Manager

## Описание

Тестовое задание для GoCloudCamp

## Реализация

+ Сервис написан на Rust
+ Конфигурации хранятся в json, схема приложения protobuf
+ Персистентность данных обеспечивается сохранением конфигов в виде json файлов
+ Поддержка CRUD операций
+ Версионирование
+ Нельзя удалить используемый конфиг

## Использование сервиса

### Создание клиента
```rust
let mut client = ConfigClient::new("http://0.0.0.0:3030").await?;
```
### Создание конфига
```rust
let mut map: HashMap<String, String> = HashMap::new();
map.insert(String::from("password"), String::from("test"));

let response = client.create_config("test", map).await?;
```
Ответ
```json
{
    "status": "Success"
}
```
### Получение конфига
```rust
let response = client.get("test").await?;
```
Ответ
```json
{
    "data": {
        "password": "test"
    }
}
```
### Получение конфига по версии
```rust
let response = client.get_version("test", 1).await?;
```
Ответ
```json
{
    "data": {
        "password": "test"
    }
}
```
### Получение всех конфигов
```rust
let response = client.get_all().await?;
```
Ответ
```json
{
  "configs": [
    {
      "service": "test",
      "data": {
        "login": "test",
        "password": "test"
      }
    }
  ]
}
```
### Апдейт конфига
```rust
let mut map: HashMap<String, String> = HashMap::new();
map.insert(String::from("login"), String::from("test"));

let response = client.update("test", map).await?;
```
Ответ
```json
{
    "status": "Config was updated"
}
```
### Удаление конфига
```rust
let response = client.delete("test").await?;
```
Ответ
```json
{
    "status": "Config was deleted"
}
```
### Удаление версии конфига
```rust
let response = client.delete("test").await?;
```
Ответ
```json
{
    "status": "Config version was deleted"
}
```
### Использование конфига
```rust
let response = client.use_config("test", 1).await?;
```
Ответ
```json
{
    "status": "Config was used"
}
```
## Запуск

### Обычный запуск
```
$ cargo run --bin config-manager-server
$ cargo run --bin config-manager-client
```
### Запуск в Docker
```
$ cargo build --bin config-manager-server --release
$ cargo build --bin config-manager-client --release
$ docker compose build
$ docker compose up -d
```


# logsrv

Требуется разработать простое web-приложение для управления записями в логе. Сервер должен обрабатывать два типа запросов:

1. Сохранить переданное сообщение в лог. Вместе с сообщением клиент передает уровень логирования (может быть "debug", "info", "warning" или "error") для данного сообщения. Сервер при сохранении также сохраняет метку времени данного сообщения (unix timestump в микро- или наносекундах).

2. Получить множество сообщений с указанным уровнем логирования, попадающих в заданный клиентом диапазон времени.

Необходимо реализовать приложение на языке Rust с использованием любых внешних библиотек для создания web-сервера и организации хранилища сообщений (любая БД или файловая система).


Запуск: 
* Создать БД в MySQL.
```shell
echo "CREATE DATABASE logsrv; GRANT ALL PRIVILEGES ON logsrv.* TO 'logsrv'@'localhost' IDENTIFIED BY 'password' WITH GRANT OPTION; FLUSH PRIVILEGES;" | mysql -u root -p 
```
* Выполнить в ней init.sql.
```shell
    cat init.sql | mysql -u logsrv -ppassword logsrv
```
* URL БД прописать в Rocket.toml.
* cargo run

Примеры обращений к серверу:
* Для добавления сообщения: curl -X POST -d "loglogloge3" http://localhost:8000/save/error
* Для извлечения сообщений: curl -X GET http://localhost:8000/get/error/2021-04-12T18:07:12.726354/2022-04-12T18:07:12.726354


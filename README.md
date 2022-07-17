# JSON-RPC API интерфейс для выставления и мониторинга ордеров на Binance
Тестовое задание для компании CoinLoan

## Использованные технологии
- Rust 1.61.0;
- PostgreSQL 14;
- Diesel ORM;
- tungstenite для подключения к WS API;
- reqwest для отправки HTTP-запросов;
- jsonrpc-http-server, jsonrpc-core, jsonrpc-derive, jsonrpc-core-client;
- serde, serde_json;
- hmac, sha2, hex для создания сигнатуры и подписи запросов.

## Пример запроса
```
curl -X GET -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "listOrders", "id": 1}' 127.0.0.1:3030
```

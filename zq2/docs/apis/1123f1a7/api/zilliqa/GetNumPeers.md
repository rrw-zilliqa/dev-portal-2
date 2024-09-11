---
id: APIs/1123f1a7/api/zilliqa/GetNumPeers
title: GetNumPeers
keywords: peers,get,number,count
---

---

!!! warning

    This API is not yet implemented in this version of Zilliqa 2.0

Returns total number of peers including committee peers.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetNumPeers",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```

### Example response

```json
{ "id": "1", "jsonrpc": "2.0", "result": 600 }
```

### Arguments

| Parameter | Type   | Required | Description       |
| --------- | ------ | -------- | ----------------- |
| `id`      | string | Required | `"1"`             |
| `jsonrpc` | string | Required | `"2.0"`           |
| `method`  | string | Required | `"GetNumPeers"`   |
| `params`  | string | Required | Empty string `""` |

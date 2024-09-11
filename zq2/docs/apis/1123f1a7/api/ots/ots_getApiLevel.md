---
id: APIs/1123f1a7/api/ots/ots_getApiLevel
title: ots_getApiLevel
keywords: ots
---

---

Returns the Otterscan API level

### Example Request

=== "cURL"

    ```shell
        curl -d '{
            "id": "1",
            "jsonrpc": "2.0",
            "method": "ots_getApiLevel",
            "params": [ ]
        }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```

### Example response

```json
{
  "jsonrpc": "2.0",
  "result": 8,
  "id": "1"
}
```

### Arguments

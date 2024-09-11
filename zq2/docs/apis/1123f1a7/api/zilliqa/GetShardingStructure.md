---
id: APIs/1123f1a7/api/zilliqa/GetShardingStructure
title: GetShardingStructure
keywords: get,sharding,structure
---
---


!!! warning

    This API is no longer relevant to Zilliqa 2.0 and will not be implemented; attempts to call it will
    result in an error. This documentation is retained for historical reasons.

Retrieves the sharding structure from the lookup server. In Zilliqa 2.0, this is replaced by the XShard on-chain query mechanism.
### Example Request


=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetShardingStructure",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```












### Example response


```json
{ "id": "1", "jsonrpc": "2.0", "result": { "NumPeers": [0] } }
```


### Arguments


| Parameter | Type   | Required | Description              |
| --------- | ------ | -------- | ------------------------ |
| `id`      | string | Required | `"1"`                    |
| `jsonrpc` | string | Required | `"2.0"`                  |
| `method`  | string | Required | `"GetShardingStructure"` |
| `params`  | string | Required | Empty string `""`        |


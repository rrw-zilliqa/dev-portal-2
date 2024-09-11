---
id: APIs/1123f1a7/api/zilliqa/GetNodeType
title: GetNodeType
keywords: node,type,get
---
---


!!! warning

    This API is not yet implemented in this version of Zilliqa 2.0

Returns node type. The possible return values are:

- `"Not in network, synced till epoch [epoch number]"` if the server has not joined the network and is synced until a specific epoch.
- `"Seed"` if the server is in lookup node mode and is an archival lookup node.
- `"Lookup"` if the server is in lookup node mode
### Example Request


=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetNodeType",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```












### Example response


```json
{ "id": "1", "jsonrpc": "2.0", "result": "Seed" }
```


### Arguments


| Parameter | Type   | Required | Description       |
| --------- | ------ | -------- | ----------------- |
| `id`      | string | Required | `"1"`             |
| `jsonrpc` | string | Required | `"2.0"`           |
| `method`  | string | Required | `"GetNodeType"`   |
| `params`  | string | Required | Empty string `""` |


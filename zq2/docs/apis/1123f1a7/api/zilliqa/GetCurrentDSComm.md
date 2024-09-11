---
id: APIs/1123f1a7/api/zilliqa/GetCurrentDSComm
title: GetCurrentDSComm
keywords: DS,committee,get,current
---

---

!!! warning

    This API is not yet implemented in this version of Zilliqa 2.0

Gives information on the public keys of DS committee members. Also, returns a parameter indicating the number of dsguards in the network.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetCurrentDSComm",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```

### Example response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": {
    "CurrentDSEpoch": "35342",
    "CurrentTxEpoch": "3534154",
    "NumOfDSGuard": 420,
    "dscomm": [
      "0x020035B739426374C5327A1224B986005297102E01C29656B8B086BF4B352C6CA9",
      "0x0200834D709AD621785A90673F6011BC36ECF4CB13475237EAA2D4DEDAE7E9E554",
      ...
    ]
  }
}
```

### Arguments

| Parameter | Type   | Required | Description          |
| --------- | ------ | -------- | -------------------- |
| `id`      | string | Required | `"1"`                |
| `jsonrpc` | string | Required | `"2.0"`              |
| `method`  | string | Required | `"GetCurrentDSComm"` |
| `params`  | string | Required | Empty string `""`    |

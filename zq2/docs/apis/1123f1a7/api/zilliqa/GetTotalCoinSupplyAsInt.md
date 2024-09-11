---
id: APIs/1123f1a7/api/zilliqa/GetTotalCoinSupplyAsInt
title: GetTotalCoinSupplyAsInt
keywords: get,coin,supply,int,total
---

---

!!! warning

    This API is not yet implemented in this version of Zilliqa 2.0

`GetTotalCoinSupplyAsInt` Returns the total supply (ZIL) of coins in the network. This is represented as a
`Rounded Number`.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetTotalCoinSupplyAsInt",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```

### Example response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": 13452081092
}
```

### Arguments

| Parameter | Type   | Required | Description                                       |
| --------- | ------ | -------- | ------------------------------------------------- |
| `id`      | string | Required | `"1"`                                             |
| `jsonrpc` | string | Required | `"2.0"`                                           |
| `method`  | string | Required | `"GetTotalCoinSupply or GetTotalCoinSupplyAsInt"` |
| `params`  | string | Required | Empty string `""`                                 |

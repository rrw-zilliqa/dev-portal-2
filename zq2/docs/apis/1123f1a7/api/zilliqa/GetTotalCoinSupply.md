---
id: APIs/1123f1a7/api/zilliqa/GetTotalCoinSupply
title: GetTotalCoinSupply
keywords: get,coin,total,supply
---
---


!!! warning

    This API is not yet implemented in this version of Zilliqa 2.0

`GetTotalCoinSupply` Returns the total supply (ZIL) of coins in the network. This is represented as a
`String`.
### Example Request


=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetTotalCoinSupply",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```



=== "node.js"

    ```js
    const totalCoinSupply = await zilliqa.blockchain.getTotalCoinSupply();
    console.log(totalCoinSupply);
    ```



=== "java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zq2-devnet.zilliqa.com");
            Rep<String> totalCoinSupply = client.getTotalCoinSupply();
            System.out.println(new Gson().toJson(totalCoinSupply));
        }
    }
    ```



=== "python"

    ```python
    from pyzil.zilliqa import chain
    from pyzil.zilliqa.api import ZilliqaAPI



=== "go"

    ```go
    func GetTotalCoinSupply() {
        provider := NewProvider("https://api.zq2-devnet.zilliqa.com")
        response := provider.GetTotalCoinSupply()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```




### Example response


```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "13452081092.277490607172"
}
```


### Arguments


| Parameter | Type   | Required | Description                                       |
| --------- | ------ | -------- | ------------------------------------------------- |
| `id`      | string | Required | `"1"`                                             |
| `jsonrpc` | string | Required | `"2.0"`                                           |
| `method`  | string | Required | `"GetTotalCoinSupply or GetTotalCoinSupplyAsInt"` |
| `params`  | string | Required | Empty string `""`                                 |


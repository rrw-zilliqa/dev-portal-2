---
id: APIs/1123f1a7/api/zilliqa/GetDSBlockRate
title: GetDSBlockRate
keywords: DS,block,get,rate
---
---


!!! warning

    This API is not yet implemented in this version of Zilliqa 2.0

Returns the current Directory Service blockrate per second.
### Example Request


=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetDSBlockRate",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```



=== "node.js"

    ```js
    const dsBlockRate = await zilliqa.blockchain.getDSBlockRate();
    console.log(dsBlockRate.result);
    ```



=== "java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zq2-devnet.zilliqa.com");
            Rep<Double> dsBlockRate = client.getDSBlockRate();
            System.out.println(new Gson().toJson(dsBlockRate));
        }
    }
    ```



=== "python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetDSBlockRate())
    ```



=== "go"

    ```go
    func GetDSBlockRate() {
        provider := NewProvider("https://api.zq2-devnet.zilliqa.com")
        response := provider.GetDSBlockRate()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```




### Example response


```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": 0.00014142137245459714
}
```


### Arguments


| Parameter | Type   | Required | Description        |
| --------- | ------ | -------- | ------------------ |
| `id`      | string | Required | `"1"`              |
| `jsonrpc` | string | Required | `"2.0"`            |
| `method`  | string | Required | `"GetDSBlockRate"` |
| `params`  | string | Required | Empty string `""`  |


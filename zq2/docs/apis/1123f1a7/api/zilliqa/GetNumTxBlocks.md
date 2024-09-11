---
id: APIs/1123f1a7/api/zilliqa/GetNumTxBlocks
title: GetNumTxBlocks
keywords: get,number,count,tx,blocks
---
---



Returns the current number of Transaction blocks in the network. This is represented as a `String`.
### Example Request


=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetNumTxBlocks",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```



=== "node.js"

    ```js
    const numTxBlock = await zilliqa.blockchain.getNumTxBlocks();
    console.log(numTxBlock.result);
    ```



=== "java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zq2-devnet.zilliqa.com");
            Rep<String> numTxBlocks = client.getNumTxBlocks();
            System.out.println(new Gson().toJson(numTxBlocks));
        }
    }
    ```



=== "python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetNumTxBlocks())
    ```



=== "go"

    ```go
    func GetNumTxBlocks() {
        provider := NewProvider("https://api.zq2-devnet.zilliqa.com")
        response := provider.GetNumTxBlocks()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
     }
     ```




### Example response


```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "589790"
}
```


### Arguments


| Parameter | Type   | Required | Description        |
| --------- | ------ | -------- | ------------------ |
| `id`      | string | Required | `"1"`              |
| `jsonrpc` | string | Required | `"2.0"`            |
| `method`  | string | Required | `"GetNumTxBlocks"` |
| `params`  | string | Required | Empty string `""`  |


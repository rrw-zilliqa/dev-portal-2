---
id: APIs/1123f1a7/api/zilliqa/GetNumDSBlocks
title: GetNumDSBlocks
keywords: DS,get,blocks,count,number
---

---

!!! warning

    This API is not yet implemented in this version of Zilliqa 2.0

Returns the current number of validated Directory Service blocks in the network. This is represented as a `String`.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetNumDSBlocks",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```

=== "node.js"

    ```js
    const numDsBlock = await zilliqa.blockchain.getNumDSBlocks();
    console.log(numDsBlock.result);
    ```

=== "java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zq2-devnet.zilliqa.com");
            Rep<String> numDSBlocks = client.getNumDSBlocks();
            System.out.println(new Gson().toJson(numDSBlocks));
        }
    }
    ```

=== "python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetNumDSBlocks())
    ```

=== "go"

    ```go
    func GetNumDSBlocks() {
        provider := NewProvider("https://api.zq2-devnet.zilliqa.com")
        response := provider.GetNumDSBlocks()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "5899"
}
```

### Arguments

| Parameter | Type   | Required | Description        |
| --------- | ------ | -------- | ------------------ |
| `id`      | string | Required | `"1"`              |
| `jsonrpc` | string | Required | `"2.0"`            |
| `method`  | string | Required | `"GetNumDSBlocks"` |
| `params`  | string | Required | Empty string `""`  |

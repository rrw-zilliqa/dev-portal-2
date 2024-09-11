---
id: APIs/1123f1a7/api/zilliqa/GetPrevDifficulty
title: GetPrevDifficulty
keywords: get,difficulty
---

---

!!! warning

    This API is no longer relevant to Zilliqa 2.0 and will not be implemented; attempts to call it will
    result in an error. This documentation is retained for historical reasons.

Returns the minimum shard difficulty of the previous block. This is represented as an `Number`. This is no longer required in Zilliqa 2.0 because of the change to proof of stake consensus.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetPrevDifficulty",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```

=== "node.js"

    ```js
    const prevDifficulty = await zilliqa.blockchain.getPrevDifficulty();
    console.log(prevDifficulty.result);
    ```

=== "java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zq2-devnet.zilliqa.com");
            Rep<Integer> prevDifficulty = client.getPrevDifficulty();
            System.out.println(new Gson().toJson(prevDifficulty));
        }
    }
    ```

=== "python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetPrevDifficulty())
    ```

=== "go"

    ```go
    func GetPrevDifficulty() {
        provider := NewProvider("https://api.zq2-devnet.zilliqa.com}")
        response := provider.GetPrevDifficulty()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": 91
}
```

### Arguments

| Parameter | Type   | Required | Description           |
| --------- | ------ | -------- | --------------------- |
| `id`      | string | Required | `"1"`                 |
| `jsonrpc` | string | Required | `"2.0"`               |
| `method`  | string | Required | `"GetPrevDifficulty"` |
| `params`  | string | Required | Empty string `""`     |

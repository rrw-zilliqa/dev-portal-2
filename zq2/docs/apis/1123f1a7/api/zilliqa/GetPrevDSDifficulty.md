---
id: APIs/1123f1a7/api/zilliqa/GetPrevDSDifficulty
title: GetPrevDSDifficulty
keywords: get,difficulty,DS
---

---

!!! warning

    This API is no longer relevant to Zilliqa 2.0 and will not be implemented; attempts to call it will
    result in an error. This documentation is retained for historical reasons.

Returns the minimum DS difficulty of the previous block. This is represented as an `Number`.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetPrevDSDifficulty",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```

=== "node.js"

    ```js
    const prevDSDifficulty = await zilliqa.blockchain.getPrevDSDifficulty();
    console.log(prevDSDifficulty.result);
    ```

=== "java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zq2-devnet.zilliqa.com");
            Rep<Integer> prevDSDifficulty = client.getPrevDSDifficulty();
            System.out.println(new Gson().toJson(prevDSDifficulty));
        }
    }
    ```

=== "python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetPrevDSDifficulty())
    ```

=== "go"

    ```go
    func GetPrevDSDifficulty() {
        provider := NewProvider("https://api.zq2-devnet.zilliqa.com")
        response := provider.GetPrevDSDifficulty()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": 149
}
```

### Arguments

| Parameter | Type   | Required | Description             |
| --------- | ------ | -------- | ----------------------- |
| `id`      | string | Required | `"1"`                   |
| `jsonrpc` | string | Required | `"2.0"`                 |
| `method`  | string | Required | `"GetPrevDSDifficulty"` |
| `params`  | string | Required | Empty string `""`       |

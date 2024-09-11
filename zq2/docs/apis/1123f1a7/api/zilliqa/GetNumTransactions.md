---
id: APIs/1123f1a7/api/zilliqa/GetNumTransactions
title: GetNumTransactions
keywords: get,transactions,count
---

---

!!! warning

    This API is not yet implemented in this version of Zilliqa 2.0

Returns the current number of validated Transactions in the network. This is represented as a `String.`

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetNumTransactions",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```

=== "node.js"

    ```js
    const numTransactions = await zilliqa.blockchain.getNumTransactions();
    console.log(numTransactions.result);
    ```

=== "java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zq2-devnet.zilliqa.com");
            Rep<String> numTransactions = client.getNumTransactions();
            System.out.println(new Gson().toJson(numTransactions));
        }
    }
    ```

=== "python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetNumTransactions())
    ```

=== "go"

    ```go
    func GetNumTransactions() {
        provider := NewProvider("https://api.zq2-devnet.zilliqa.com")
        response := provider.GetNumTransactions()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "4350695"
}
```

### Arguments

| Parameter | Type   | Required | Description            |
| --------- | ------ | -------- | ---------------------- |
| `id`      | string | Required | `"1"`                  |
| `jsonrpc` | string | Required | `"2.0"`                |
| `method`  | string | Required | `"GetNumTransactions"` |
| `params`  | string | Required | Empty string `""`      |

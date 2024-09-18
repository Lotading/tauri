<script>
import { get_port, testconn } from "$lib/testconn";

    /**
     * @type {Number}
     */
let port;

get_port().then(value => {
  port = value;
});

/**
 * @type {any}
 */
let result = '';


async function testConnection() {
  try {
    const response = await testconn();
    // @ts-ignore
    result = await response;
  } catch (err) {
    // @ts-ignore
    result = err.message;
  }
}
</script>

<p>{ port }</p>
<button on:click={testConnection}>Test Connection</button>

<p>Connection successful: {result}</p>
<h1>
  main
</h1>

<script>
import {testconn} from '$lib/testconn';

export async function get() {
  try {
    const response = await testconn();
    console.log(response)
  } catch (error) {
    console.error(error)
  }
}
get()
</script>
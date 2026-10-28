<h1>This is a test bench for me using said languages to read, parse, and serve HTML.</h1>
<p>Allows us to write interesting things aside, it helps us with control over web servers like NginX or Apache, by eliminating unpredictable or unforeseen traffic / overhead. </p>
<p>Allows us to micro manage things like network and socket behaviour compared to regular Nginx/Apache.</p>
<p>Allows us to understand how many users can be using the app concurrently before things start spiralling out of control</p>
<p>It simulates worst case scenarios.</p>
<ol>
  <h1>When it comes to performance</h1>
  <li>C++: ~86k Req/S</li>
  <li>Rust: ~67k Req/S</li>
  <li>Go: ~62k Req/s</li>
</ol>

<p>Go was the simplest to get up and running, but with GC instead of manual memory management, tends to be slower. Rust and C++ are faster but requires a lot more fine tuning and optimization such as optimizing compile flags. But I don't believe that Go and Rust should have performances this close, will fine tune rust to get better performance out of it as it seems a bit off.</p>

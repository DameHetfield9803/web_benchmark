#include "crow.h"
#include <algorithm>
#include <iostream>
#include <string>
#include <thread>

int main()
{
    crow::logger::setLogLevel(crow::LogLevel::Warning);

    crow::SimpleApp app;

    // Load templates once at startup.
    const std::string index_page =
        crow::mustache::load_text("index.html");

    const auto greet_template =
        crow::mustache::load("greet.html");

    // Root page.
    CROW_ROUTE(app, "/")([&index_page]() {
        return index_page;
    });

    // Plain-text endpoint.
    CROW_ROUTE(app, "/hello")([]() {
        return "Hello\n";
    });

    // Dynamic greeting.
    CROW_ROUTE(app, "/<string>")(
        [&greet_template](const std::string& name) {
            crow::mustache::context ctx({
                {"person", name}
            });

            return greet_template.render(ctx);
        }
    );

    // User list.
    CROW_ROUTE(app, "/api/users")([]() {
        return "1. John\n2. Mary\n3. Jack";
    });

    // hardware_concurrency() may return zero.
    const unsigned int threads = std::max(
        1u, std::thread::hardware_concurrency()
    );

    std::cout << "Server starting on port 3000\n";

    app.port(3000)
       .concurrency(threads)
       .tcp_nodelay(true)
       .run();
}

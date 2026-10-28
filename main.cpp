 #include "crow.h"


int main()
{
    crow::logger::setLogLevel(crow::LogLevel::Warning);

    crow::SimpleApp app; //define your crow application

    //define your endpoint at the root directory

    CROW_ROUTE(app, "/")([](){
        auto page = crow::mustache::load_text("index.html");
        return page;
    });

    CROW_ROUTE(app, "/hello")([](){
        return "Hello \n";
    });

    CROW_ROUTE(app, "/<string>")([](std::string name){
            auto page = crow::mustache::load("greet.html");
            crow::mustache::context ctx({{"person", name}});
            return page.render(ctx);
    });

    CROW_ROUTE(app, "/api/users")([](){
        return "1. John\n2. Mary\n3. Jack";
    });

    unsigned int threads = std::thread::hardware_concurrency();

    app.tcp_nodelay(true).port(3000).concurrency(threads).run();

}

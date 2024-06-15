use dotenv::dotenv;
use rand::Rng;
use redis::Commands;
use std::collections::BTreeMap;
use std::env;

fn main() {
    basics();
    hash();
    list();
    set();
    sorted_set();
}

fn connect() -> redis::Connection {
    dotenv().ok();
    //format - host:port
    let redis_host_name = match env::var("REDIS_HOSTNAME") {
        Ok(val) => val,
        Err(_) => panic!("missing environment variable REDIS_HOSTNAME"),
    };
    let redis_password = match env::var("REDIS_PASSWORD") {
        Ok(val) => val,
        Err(_) => "".to_string(),
    };

    //if Redis server needs secure connection
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };

    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    println!("{}", redis_conn_url);

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

fn basics() {
    let mut conn = connect();
    // Strings
    // SET key value: Set the string value of a key.
    // GET key: Get the value of a key.
    // DEL key: Delete a key.
    // INCR key: Increment the integer value of a key by one.
    // DECR key: Decrement the integer value of a key by one.
    println!("******* Running SET, GET, INCR commands *******");

    let _: () = redis::cmd("SET")
        .arg("foo")
        .arg("bar")
        .query(&mut conn)
        .expect("failed to execute SET for 'foo'");

    let bar: String = redis::cmd("GET")
        .arg("foo")
        .query(&mut conn)
        .expect("failed to execute GET for 'foo'");
    println!("value for 'foo' = {}", bar);
    //delete a key
    let _del_bar: bool = redis::cmd("DEL")
        .arg("foo")
        .query(&mut conn)
        .expect("failed to execute DEL for 'foo'");

    //INCR and GET using high-level commands
    let _: () = conn
        .incr("counter", 2)
        .expect("failed to execute INCR for 'counter'");
    let val: i32 = conn
        .get("counter")
        .expect("failed to execute GET for 'counter'");
    println!("counter = {}", val);
    let _: () = conn
        .decr("counter", 2)
        .expect("failed to execute INCR for 'counter'");

    let val: i32 = conn
        .get("counter")
        .expect("failed to execute GET for 'counter'");

    println!("counter = {}", val);
}

fn hash() {
    let mut conn = connect();
    // Hashes
    // HSET key field value: Set the hash field to the specified value.
    // HGET key field: Get the value of a hash field.
    // HDEL key field [field ...]: Delete one or more hash fields.
    // HGETALL key: Get all fields and values in a hash.

    println!("******* Running HASH commands *******");

    let mut driver: BTreeMap<String, String> = BTreeMap::new();
    let prefix = "redis-driver";

    driver.insert(String::from("name"), String::from("redis-rs"));
    driver.insert(String::from("version"), String::from("0.19.0"));
    driver.insert(
        String::from("repo"),
        String::from("https://github.com/mitsuhiko/redis-rs"),
    );

    let _: () = redis::cmd("HSET")
        .arg(format!("{}:{}", prefix, "rust"))
        .arg(driver)
        .query(&mut conn)
        .expect("failed to execute HSET");

    let info: BTreeMap<String, String> = redis::cmd("HGETALL")
        .arg(format!("{}:{}", prefix, "rust"))
        .query(&mut conn)
        .expect("failed to execute HGETALL");

    println!("info for rust redis driver: {:?}", info);

    let _: () = conn
        .hset_multiple(
            format!("{}:{}", prefix, "go"),
            &[
                ("name", "go-redis"),
                ("version", "8.4.6"),
                ("repo", "https://github.com/go-redis/redis"),
            ],
        )
        .expect("failed to execute HSET");

    let repo_name: String = conn
        .hget(format!("{}:{}", prefix, "go"), "repo")
        .expect("failed to execute HGET");

    println!("go redis driver repo name: {:?}", repo_name);
}

fn list() {
    let mut conn = connect();
    // Lists
    // LPUSH key value [value ...]: Insert one or multiple values at the beginning of a list.
    // RPUSH key value [value ...]: Insert one or multiple values at the end of a list.
    // LPOP key: Remove and get the first element in a list.
    // RPOP key: Remove and get the last element in a list.
    // LRANGE key start stop: Get a range of elements from a list.

    println!("******* Running LIST commands *******");

    let list_name = "items";

    let _: () = redis::cmd("LPUSH")
        .arg(list_name)
        .arg("item-1")
        .query(&mut conn)
        .expect("failed to execute LPUSH for 'items'");

    let item: String = conn
        .lpop(list_name, None)
        .expect("failed to execute LPOP for 'items'");
    println!("first item: {}", item);

    let _: () = conn.rpush(list_name, "item-2").expect("RPUSH failed");
    let _: () = conn.rpush(list_name, "item-3").expect("RPUSH failed");

    let len: isize = conn
        .llen(list_name)
        .expect("failed to execute LLEN for 'items'");
    println!("no. of items in list = {}", len);

    let items: Vec<String> = conn
        .lrange(list_name, 0, len - 1)
        .expect("failed to execute LRANGE for 'items'");
    println!("listing items in list");

    for item in items {
        println!("item: {}", item)
    }
}

fn set() {
    let mut conn = connect();
    // Sets
    // SADD key member [member ...]: Add one or more members to a set.
    // SMEMBERS key: Get all members of a set.
    // SREM key member [member ...]: Remove one or more members from a set.
    // SISMEMBER key member: Check if a member exists in a set.
    println!("******* Running SET commands *******");

    let set_name = "users";

    let _: () = conn
        .sadd(set_name, "user1")
        .expect("failed to execute SADD for 'users'");
    let _: () = conn
        .sadd(set_name, "user2")
        .expect("failed to execute SADD for 'users'");

    let ismember: bool = redis::cmd("SISMEMBER")
        .arg(set_name)
        .arg("user1")
        .query(&mut conn)
        .expect("failed to execute SISMEMBER for 'users'");
    println!("does user1 exist in the set? {}", ismember); //true

    let users: Vec<String> = conn.smembers(set_name).expect("failed to execute SMEMBERS");
    println!("listing users in set"); //true

    for user in users {
        println!("user: {}", user)
    }
}

fn sorted_set() {
    let mut conn = connect();
    // Sorted Sets
    // ZADD key score member [score member ...]: Add one or more members to a sorted set, or update the score of existing members.
    // ZRANGE key start stop [WITHSCORES]: Get a range of members from a sorted set.
    // ZSCORE key member: Get the score of a member in a sorted set.
    // ZREM key member [member ...]: Remove one or more members from a sorted set.
    println!("******* Running SORTED SET commands *******");

    let sorted_set = "leaderboard";

    let _: () = redis::cmd("ZADD")
        .arg(sorted_set)
        .arg(rand::thread_rng().gen_range(1..10))
        .arg("player-1")
        .query(&mut conn)
        .expect("failed to execute ZADD for 'leaderboard'");

    //add many players
    for num in 2..=5 {
        let _: () = conn
            .zadd(
                sorted_set,
                String::from("player-") + &num.to_string(),
                rand::thread_rng().gen_range(1..10),
            )
            .expect("failed to execute ZADD for 'leaderboard'");
    }

    let count: isize = conn
        .zcard(sorted_set)
        .expect("failed to execute ZCARD for 'leaderboard'");

    let leaderboard: Vec<(String, isize)> = conn
        .zrange_withscores(sorted_set, 0, count - 1)
        .expect("ZRANGE failed");
    println!("listing players and scores");

    for item in leaderboard {
        println!("{} = {}", item.0, item.1)
    }
}

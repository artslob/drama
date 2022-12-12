package com.github.artslob.drama;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.boot.test.context.SpringBootTest.WebEnvironment;
import org.springframework.boot.test.web.server.LocalServerPort;

@SpringBootTest(webEnvironment = WebEnvironment.RANDOM_PORT)
// @Import(RedditWireMockExtension.class)
@ExtendWith(RedditWireMockExtension.class)
public class TestWithWireMock {
    @LocalServerPort
    private int port;
    //    @Autowired
    //    private MainProperties properties;

    @Test
    public void testing() {}
}

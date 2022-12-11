package com.github.artslob.drama;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.boot.test.context.SpringBootTest.WebEnvironment;
import org.springframework.boot.test.web.client.TestRestTemplate;
import org.springframework.boot.test.web.server.LocalServerPort;

@SpringBootTest(webEnvironment = WebEnvironment.RANDOM_PORT)
class DramaApplicationTests {

    @LocalServerPort
    private int port;

    @Autowired
    private TestRestTemplate restTemplate;

    @Test
    void rootResponse() {
        var response = restTemplate.getForObject("http://localhost:" + port + "/", String.class);
        assertThat(response)
                .contains("client_id=")
                .contains("redirect_uri=")
                .contains("state=")
                .contains("scope=")
                .contains("duration=permanent");
    }
}

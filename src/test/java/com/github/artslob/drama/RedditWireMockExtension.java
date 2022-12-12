package com.github.artslob.drama;

import static com.github.tomakehurst.wiremock.client.WireMock.okJson;
import static com.github.tomakehurst.wiremock.client.WireMock.post;
import static com.github.tomakehurst.wiremock.client.WireMock.urlPathEqualTo;
import static com.github.tomakehurst.wiremock.core.WireMockConfiguration.wireMockConfig;

import com.github.artslob.drama.properties.MainProperties;
import com.github.tomakehurst.wiremock.WireMockServer;
import org.junit.jupiter.api.extension.AfterAllCallback;
import org.junit.jupiter.api.extension.BeforeAllCallback;
import org.junit.jupiter.api.extension.ExtensionContext;
import org.springframework.boot.test.context.TestConfiguration;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Primary;

@TestConfiguration
public class RedditWireMockExtension implements BeforeAllCallback, AfterAllCallback {

    private final WireMockServer server = new WireMockServer(wireMockConfig().dynamicPort());

    @Bean
    @Primary
    public MainProperties updateProperties() {
        var props = new MainProperties();
        props.access_token_url = String.format("%s/api/v1/access_token", server.baseUrl());
        return props;
    }

    @Override
    public void beforeAll(ExtensionContext context) throws Exception {
        server.start();

        server.stubFor(
                post(urlPathEqualTo("api/v1/access_token"))
                        .willReturn(
                                okJson(
                                        """
                {
                    "access_token": "qwe",
                    "token_type": "qwe",
                    "expires_in": 3600,
                    "scope": "qwe",
                    "refresh_token": "qwe"
                }
                """)));
    }

    @Override
    public void afterAll(ExtensionContext context) throws Exception {
        server.stop();
    }
}

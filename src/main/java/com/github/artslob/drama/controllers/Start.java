package com.github.artslob.drama.controllers;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class Start {
    @GetMapping("/")
    public String index() {
        var scope = "identity,history,mysubreddits,read";
        var redirect_uri = "http://localhost:8080/callback";
        var state = "64990aeb-5178-43d3-8ccb-110962843622";
        // TODO remove this application later
        var applicationId = "giud55ItUqIbi591qrFl_A";
        var url = String.format(
                """
                        https://www.reddit.com/api/v1/authorize?\
                        client_id=%s\
                        &response_type=code&state=%s\
                        &redirect_uri=%s\
                        &duration=permanent\
                        &scope=%s\
                        """,
                applicationId,
                state,
                redirect_uri,
                scope
        );
        return String.format("<a href=\"%s\">go here</a>", url);
    }

    @GetMapping("/callback")
    public String callback(
            @RequestParam(required = false) String error,
            @RequestParam(required = false) String code,
            @RequestParam(required = false) String state
    ) {
        if (error != null) {
            return String.format("error occurred :( : %s", error);
        }
        if (code == null || state == null) {
            return "got empty code or state :(";
        }
        System.out.println(code);
        System.out.println(state);
        return "success";
    }
}

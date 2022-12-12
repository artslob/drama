package com.github.artslob.drama;

import static org.hamcrest.Matchers.containsString;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.content;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.web.servlet.MockMvc;

@SpringBootTest
@AutoConfigureMockMvc
public class TestWithMockMvc {
    @Autowired
    private MockMvc mockMvc;

    @Test
    public void checkRoot() throws Exception {
        mockMvc.perform(get("/")).andExpect(status().isOk()).andExpect(content().string(containsString("client_id=")));
    }

    //    @Test
    //    public void checkCallback() throws Exception {
    //        mockMvc.perform(get("/callback").queryParam("code", "qwe").queryParam("state", "qwe"))
    //                .andExpect(status().isOk())
    //                .andExpect(content().string(containsString("success")));
    //    }
}

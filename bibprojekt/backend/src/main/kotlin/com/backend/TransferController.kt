package com.backend

import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestMapping


@org.springframework.web.bind.annotation.RestController
@RequestMapping("/transfer")
class TransferController {

    @PostMapping("/new")
    fun newTransfer() {
        System.out.println("Saved")
    }

    @GetMapping("/all")
    fun getAllTransfers(): String {
        return "greetings Sir"
    }

}
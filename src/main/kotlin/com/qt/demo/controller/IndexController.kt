package com.qt.demo.controller

import com.qt.demo.common.R
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController
import org.springframework.web.multipart.MultipartFile
import java.nio.file.Path


@RestController
@RequestMapping("/index")
class IndexController {

    @GetMapping("/{name}")
    fun index(@PathVariable name: String): String {
        return "hello, $name"
    }

    @PostMapping("/upload")
    fun upload(files: Array<MultipartFile>): R<*> {
        for (file in files) {
            println(file.originalFilename)
            file.transferTo(Path.of("./upload/${file.originalFilename}"))
        }
        return R.success<String>("upload success")
    }
}
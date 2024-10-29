package com.qt.demo.common;

public enum BizCode implements IBizCode {
    Ok(0, "Success"),
    Error(10, "Error"),

    ArgumentNotValid(100, "ArgumentNotValid"),
    ;
    private final int code;
    private final String msg;

    BizCode(int code, String msg) {
        this.code = code;
        this.msg = msg;
    }

    @Override
    public int getCode() {
        return code;
    }

    @Override
    public String getMessage() {
        return msg;
    }
}

#pragma once

#include <ntcore_c.h>

struct NTCoreRS_Vec {
    char* data;
    size_t len;
    size_t capacity;
};

enum NTCoreRS_AllocType {
    // Alloc a bool
    Bool = 1,
    // Alloc a f64
    Double = 2,
    // Alloc an array of strings
    String = 4,
    // Alloc a u8/i8 vec
    Char = 8,
    // Alloc an i64
    Integer = 0x100,
    // Alloc an f32
    Float = 0x200,
    // alloc an NTCoreRS_Value
    Value = 0x100000,
    // alloc an NT_Handle
    Handle = 0x200000,
};

union NTCoreRS_ValueType {
    bool v_boolean;
    int64_t v_int;
    float v_float;
    double v_double;
    NTCoreRS_Vec buf;
};

struct NTCoreRS_Value {
    NT_Type type;
    int64_t last_change;
    int64_t server_time;
    NTCoreRS_ValueType data;
};

struct NTCoreRS_TopicInfo {
    NT_Topic topic;
    NTCoreRS_Vec name;
    NT_Type type;
    NTCoreRS_Vec type_str;
    NTCoreRS_Vec properties;
};


/** Pointer to the vec allocator function */
typedef NTCoreRS_Vec (*NTCoreRS_Allocator)(NTCoreRS_AllocType, const void*, size_t);

/** Pointer to a function that will convert from the opaque nt::Value to an NTCoreRS_Value */
typedef NTCoreRS_Value (*NTCoreRS_Value_Convert)(NTCoreRS_Allocator, const void*);
typedef NTCoreRS_Vec (*NTCoreRS_ReadQueue_Construct)(NTCoreRS_Value_Convert, const void*, size_t);

typedef NTCoreRS_TopicInfo (*NTCoreRS_TopicInfo_Convert)(NTCoreRS_Allocator, const void*);
typedef NTCoreRS_Vec (*NTCoreRS_InstTopicInfo_Construct)(NTCoreRS_TopicInfo_Convert, const void*, size_t);


extern "C" {
    /**
     * GetEntry shim
     */
    NT_Entry NTCoreRS_GetEntry(NT_Inst inst, const char* str, size_t str_len);
    NT_Topic NTCoreRS_GetTopic(NT_Inst inst, const char* str, size_t str_len);

    NTCoreRS_Vec NTCoreRS_GetEntryName(NT_Entry entry, NTCoreRS_Allocator alloc);
    NTCoreRS_Value NTCoreRS_GetEntryValue(NT_Handle subentry, NTCoreRS_Allocator alloc);
    NTCoreRS_Vec NTCoreRS_ReadQueueValue(
        NT_Handle subentry,
        unsigned int types,
        NTCoreRS_Allocator alloc,
        NTCoreRS_ReadQueue_Construct construct
    );
    NTCoreRS_Vec NTCoreRS_GetTopics(NT_Inst inst, const char* prefix, size_t prefix_len, unsigned int types, NTCoreRS_Allocator alloc);
    NTCoreRS_Vec NTCoreRS_GetTopicsStr(NT_Inst inst, const char* prefix, size_t prefix_len, const WPI_String* types, size_t types_len, NTCoreRS_Allocator alloc);

    NTCoreRS_Vec NTCoreRS_GetTopicInfos(NT_Inst inst, const char* prefix, size_t prefix_len, unsigned int types, NTCoreRS_Allocator alloc, NTCoreRS_InstTopicInfo_Construct construct);
    NTCoreRS_Vec NTCoreRS_GetTopicInfosStr(NT_Inst inst, const char* prefix, size_t prefix_len, const WPI_String* types, size_t types_len, NTCoreRS_Allocator alloc, NTCoreRS_InstTopicInfo_Construct construct);

    NTCoreRS_TopicInfo NTCoreRS_GetTopicInfo(NT_Topic topic, NTCoreRS_Allocator alloc);
    NTCoreRS_Vec NTCoreRS_GetTopicName(NT_Topic topic, NTCoreRS_Allocator alloc);
    NTCoreRS_Vec NTCoreRS_GetTopicTypeString(NT_Topic topic, NTCoreRS_Allocator alloc);



}
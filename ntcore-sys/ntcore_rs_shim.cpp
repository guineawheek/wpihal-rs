#include "ntcore_cpp.h"
#include "NTCoreShim.h"
#include <string_view>

static NTCoreRS_Vec StringViewToVec(std::string_view in, NTCoreRS_Allocator alloc) {
    if (in.empty()) {
        return alloc(NTCoreRS_AllocType::Char, NULL, 0);
    }
    return alloc(NTCoreRS_AllocType::Char, in.data(), in.size());
}

static NTCoreRS_Value ConvertValue(const nt::Value& value, NTCoreRS_Allocator alloc) {
    NTCoreRS_Value ret_value{};
    ret_value.type = value.type();
    ret_value.last_change = value.last_change();
    ret_value.server_time = value.server_time();

    switch (ret_value.type) {
        case NT_Type::NT_UNASSIGNED: {
            break;
        }
        case NT_Type::NT_BOOLEAN: {
            ret_value.data.v_boolean = value.value().data.v_boolean;
            break;
        }
        case NT_Type::NT_DOUBLE: {
            ret_value.data.v_double = value.value().data.v_double;
            break;
        }
        case NT_Type::NT_STRING: {
            ret_value.data.buf = alloc(
                NTCoreRS_AllocType::Char,
                value.value().data.v_string.str,
                value.value().data.v_string.len
            );
            break;
        }
        case NT_Type::NT_RAW: 
        case NT_Type::NT_RPC:
        {
            ret_value.data.buf = alloc(
                NTCoreRS_AllocType::Char,
                value.value().data.v_raw.data,
                value.value().data.v_raw.size
            );
            break;
        }
        case NT_Type::NT_BOOLEAN_ARRAY: {
            ret_value.data.buf = alloc(
                NTCoreRS_AllocType::Bool,
                value.value().data.arr_boolean.arr,
                value.value().data.arr_boolean.size
            );
            break;
        }
        case NT_Type::NT_DOUBLE_ARRAY: {
            ret_value.data.buf = alloc(
                NTCoreRS_AllocType::Double,
                value.value().data.arr_double.arr,
                value.value().data.arr_double.size
            );
            break;
        }
        case NT_Type::NT_STRING_ARRAY: {
            ret_value.data.buf = alloc(
                NTCoreRS_AllocType::String,
                value.value().data.arr_string.arr,
                value.value().data.arr_string.size
            );
            break;
        }
        case NT_Type::NT_INTEGER: {
            ret_value.data.v_int = value.value().data.v_int;
            break;
        }
        case NT_Type::NT_FLOAT: {
            ret_value.data.v_float = value.value().data.v_float;
            break;
        }
        case NT_Type::NT_INTEGER_ARRAY: {
            ret_value.data.buf = alloc(
                NTCoreRS_AllocType::Integer,
                value.value().data.arr_int.arr,
                value.value().data.arr_int.size
            );
            break;
        }
        case NT_Type::NT_FLOAT_ARRAY: {
            ret_value.data.buf = alloc(
                NTCoreRS_AllocType::Float,
                value.value().data.arr_float.arr,
                value.value().data.arr_float.size
            );
            break;
        }
    }

    return ret_value;
}

static NTCoreRS_Value Value_Convert(NTCoreRS_Allocator alloc, const void* value) {
    nt::Value* v = (nt::Value*) value;
    return ConvertValue(*v, alloc);
}

static NTCoreRS_TopicInfo TopicInfo_Convert(NTCoreRS_Allocator alloc, const void* value) {
    nt::TopicInfo* v = (nt::TopicInfo*) value;
    NTCoreRS_TopicInfo ret;
    ret.topic = v->topic;
    ret.name = alloc(NTCoreRS_AllocType::String, v->name.data(), v->name.size());
    ret.type = v->type;
    ret.type_str = alloc(NTCoreRS_AllocType::String, v->type_str.data(), v->type_str.size());
    ret.properties = alloc(NTCoreRS_AllocType::String, v->properties.data(), v->properties.size());
    return ret;
}


extern "C" {

NT_Entry NTCoreRS_GetEntry(NT_Inst inst, const char* str, size_t str_len) {
    return nt::GetEntry(inst, {str, str_len});
}

NT_Topic NTCoreRS_GetTopic(NT_Inst inst, const char* str, size_t str_len) {
    return nt::GetTopic(inst, {str, str_len});
}

NTCoreRS_Vec NTCoreRS_GetEntryName(NT_Entry entry, NTCoreRS_Allocator alloc) {
    return StringViewToVec(nt::GetEntryName(entry), alloc);
}

NT_Type NTCoreRS_GetEntryType(NT_Entry entry) {
    return nt::GetEntryType(entry);
}

int64_t NTCoreRS_GetEntryLastChange(NT_Handle subentry) {
    return nt::GetEntryLastChange(subentry);
}

NTCoreRS_Value NTCoreRS_GetEntryValue(NT_Handle subentry, NTCoreRS_Allocator alloc) {
    nt::Value value = nt::GetEntryValue(subentry);
    return ConvertValue(value, alloc);
}

NTCoreRS_Vec NTCoreRS_ReadQueueValue(
    NT_Handle subentry,
    unsigned int types,
    NTCoreRS_Allocator alloc,
    NTCoreRS_ReadQueue_Construct construct
) {
    std::vector<nt::Value> values = nt::ReadQueueValue(subentry, types);
    return construct(Value_Convert, values.data(), values.size());
}

NTCoreRS_Vec NTCoreRS_GetTopics(NT_Inst inst, const char* prefix, size_t prefix_len, unsigned int types, NTCoreRS_Allocator alloc) {
    std::vector<NT_Topic> topics = nt::GetTopics(inst, {prefix, prefix_len}, types);
    return alloc(NTCoreRS_AllocType::Handle, topics.data(), topics.size());
}

NTCoreRS_Vec NTCoreRS_GetTopicsStr(NT_Inst inst, const char* prefix, size_t prefix_len, const WPI_String* types, size_t types_len, NTCoreRS_Allocator alloc) {
    std::vector<std::string_view> typesCpp;
    typesCpp.reserve(types_len);
    for (size_t i = 0; i < types_len; ++i) {
        typesCpp.emplace_back(wpi::to_string_view(&types[i]));
    }
    auto topics = nt::GetTopics(inst, {prefix, prefix_len}, typesCpp);
    return alloc(NTCoreRS_AllocType::Handle, topics.data(), topics.size());
}

NTCoreRS_Vec NTCoreRS_GetTopicInfos(NT_Inst inst, const char* prefix, size_t prefix_len, unsigned int types, NTCoreRS_Allocator alloc, NTCoreRS_InstTopicInfo_Construct construct) {
    std::vector<nt::TopicInfo> values = nt::GetTopicInfo(inst, {prefix, prefix_len}, types);
    return construct(TopicInfo_Convert, values.data(), values.size());
}

NTCoreRS_Vec NTCoreRS_GetTopicInfosStr(NT_Inst inst, const char* prefix, size_t prefix_len, const WPI_String* types, size_t types_len, NTCoreRS_Allocator alloc, NTCoreRS_InstTopicInfo_Construct construct) {
    std::vector<std::string_view> typesCpp;
    typesCpp.reserve(types_len);
    for (size_t i = 0; i < types_len; ++i) {
        typesCpp.emplace_back(wpi::to_string_view(&types[i]));
    }
    std::vector<nt::TopicInfo> values = nt::GetTopicInfo(inst, {prefix, prefix_len}, typesCpp);
    return construct(TopicInfo_Convert, values.data(), values.size());
}

NTCoreRS_TopicInfo NTCoreRS_GetTopicInfo(NT_Topic topic, NTCoreRS_Allocator alloc) {
    auto topic = nt::GetTopicInfo(topic);
    return TopicInfo_Convert(alloc, &topic);
}

NTCoreRS_Vec NTCoreRS_GetTopicName(NT_Topic topic, NTCoreRS_Allocator alloc) {
    auto name = nt::GetTopicName(topic);
    return alloc(NTCoreRS_AllocType::String, name.data(), name.size());
}

NTCoreRS_Vec NTCoreRS_GetTopicTypeString(NT_Topic topic, NTCoreRS_Allocator alloc) {
    auto s = nt::GetTopicTypeString(topic);
    return alloc(NTCoreRS_AllocType::String, s.data(), s.size());

}

}
//
// Created by lucas on 28/01/2024.
//
#include <stdio.h>
#include <Windows.h>

#define CHECK_NULL(x) if (x == NULL) { printf("GetProcAddress failed\n"); return 1; }
# define CHECK_ERROR(x) if (x != 0) { printf("error\n"); return x; }

int main(int argc, char * argv[])
{
    HMODULE libHandle;

    if ((libHandle = LoadLibrary(TEXT("map_c_lib.dll"))) == NULL)
    {
        printf("load failed\n");
        return 1;
    }

    int (*map_init)() = (int (*)())GetProcAddress(libHandle, "map_init");
    CHECK_NULL(map_init);

    // node id
    int (*add_node)(int) = (int (*)(int))GetProcAddress(libHandle, "add_node");
    CHECK_NULL(add_node);

    // switch id
    int (*add_switch)(int) = (int (*)(int))GetProcAddress(libHandle, "add_switch");
    CHECK_NULL(add_switch);

    // switch, position_center, position_diverted, position_straight
    int (*add_switch_station)(int,int,int,int) = (int (*)(int,int,int,int))GetProcAddress(libHandle, "add_switch_station");
    CHECK_NULL(add_switch_station);

    // position_1, position_2, max_speed_1_to_2, max_speed_2_to_1, length
    int (*add_link)(int,int,char,char,int) = (int (*)(int,int,char,char,int))GetProcAddress(libHandle, "add_link");
    CHECK_NULL(add_link);

    // train, position, pointing_to
    int (*add_train)(int,int,int) = (int (*)(int,int,int))GetProcAddress(libHandle, "add_train");
    CHECK_NULL(add_train);

    // buffer, size
    int (*get_map_json)(char *, unsigned int) = (int (*)(char *, unsigned int))GetProcAddress(libHandle, "get_map_json");
    CHECK_NULL(get_map_json);

    CHECK_ERROR(map_init());

    CHECK_ERROR(add_node(1));
    CHECK_ERROR(add_node(2));
    CHECK_ERROR(add_node(3));
    CHECK_ERROR(add_node(4));
    CHECK_ERROR(add_node(5));

    CHECK_ERROR(add_switch(1));

    CHECK_ERROR(add_switch_station(1, 1, 2, 3));

    CHECK_ERROR(add_link(1, 4, 1, 2, 10));
    CHECK_ERROR(add_link(2, 5, 1, 2, 10));

    CHECK_ERROR(add_train(1, 4, -1));
    CHECK_ERROR(add_train(2, 5, 2));

    char buffer[5000];
    CHECK_ERROR(get_map_json(buffer, 5000));

    printf("%s\n", buffer);

    return 0;
}
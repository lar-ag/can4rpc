

typedef unsigned int U32;
typedef unsigned char U8;
typedef int Err;
typedef unsigned char TYPE;

Err      close_can0();
U32      read_unsigned(int node,int index,U8 sub);
Err      write_unsigned(int node,int index,U8 sub, TYPE type, U32 value);
char*    analog_get_uart01(int node);
Err      analog_set_baut01(int node,U32 baut);
Err      analog_set_uart01(int node,char *data);
char*    analog_get_uart02(int node);
Err      analog_set_baut02(int node,U32 baut);
Err      analog_set_uart02(int node,char *data);
U32      analog_get_in01  (int node);
U32      analog_get_in02 (int node);
U32      analog_get_in03  (int node);
U32      analog_get_in04  (int node);
U32      analog_get_in05  (int node);
U32      analog_get_out   (int node);
Err      analog_set_out   (int node,U32 value);
U32      analog_get_temp01(int node);
U32      analog_get_temp02(int node);
U32      analog_get_temp03(int node);

#include <fcntl.h>
#include <malloc.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include "can4linux.h"
#include "can.h"


typedef struct Data{
    unsigned char buf[256];
    unsigned char len;
}Data;

///  Typed:
///     0x001 - bool
///     0x002 - u8
///     0x003 - u16
///     0x004 - i16
///     0x006 - i16

static int can0() {
  static int fd = 0;
  if(fd == 0) {
    	fd = open ("/dev/can0", O_RDWR | O_NONBLOCK);
  }
  return fd;
}

Err close_can0() {
  int fd = can0();
  return close(fd);
}


static Err read_string(int fd,int node, canmsg_t *rx, Data *data) {
    unsigned char len = rx->data[4];
    data->len = 0;
    canmsg_t msg;
    msg.flags = 0;
    msg.cob   = 0;
    msg.id    = (node & 0x7F) | 0x600;
    msg.data[0] =  0x60;

    while (len <= data->len)
    {

       	if(write (fd, &msg, 1)) return 3;
        canmsg_t nrx;
        if(read (fd, &nrx, 1)) return 2;
        int l = 7 - ((nrx.data[0] & 0xE)>>1);
        int i =1;
        for(i=1;i<=l;i++) {
            data->buf[data->len] = nrx.data[i];
            data->len++;
        }
        msg.data[0] ^= 0x10;
        if(l<7)break;
    }
    return 0;
}
static Err write_string(int fd,int node, Data *data) {
    unsigned char len = data->len;
    canmsg_t msg;
    canmsg_t nrx;

    msg.flags = 0;
	msg.cob   = 0;
    msg.length   = 8;
	msg.id    = (node & 0x7F) | 0x600;
    unsigned toggle = 0x0;
    unsigned pos = 0;
    while (len)
    {
        int l = len>7?7:len;
        msg.data [0] = toggle | (14-2*l) | (len<=7);
        int i = 1;
        for(i=1;i<=l;i++) {
            msg.data[i] = data->buf[pos];
            pos ++;
        }
       	if(write (fd, &msg, 1)) return 3;
        if(read (fd, &nrx, 1)) return 2;
        toggle ^= 0x10;
        len -=l;
    }
    return 0;
}



static Err set_value(canmsg_t *rx, Data *data) {
    if (0x40 != (rx->data[0]&0xE0))
		return 0;
	if (2 & rx->data[0]) {
		// 0: 4, 1: 3, 2: 2, 3: 1
		data->len   = 4 - (3 & (rx->data[0] >> 2));
		data->buf[0]  = rx -> data[4];

		if ( data->len > 1)
			 data->buf[1] = rx->data[5];

		if (data->len > 2)
			 data->buf[2] = rx->data[6];

		if (data->len > 3)
			 data->buf[3] = rx->data[7];
		// Es wurde eine Zahl der Länge '* size' mit dem Wert '* value' gelesen.
		return 0;
	}

	if (0x41 == rx->data[0]) {
		data->len  = rx -> data[4] | rx->data[5] << 8;
		return 0;
	}
	return 2;
}

// cmd = 0x20|19-(len<<2)
static Err can_msg (int fd, int node, int index, unsigned char sub, Data *data)
{
  static Data buffer;
  if(data == NULL) {
    buffer.len =0;
    memset(buffer.buf,0,256);
    data = &buffer;
  }
	canmsg_t msg;
    switch (data->len)
    {
    case 0:
        msg.data [0] = 0x40;
        break;
    case 1:
        msg.data [0] = 0x2F;
        msg.data[4]  = data->buf[0];
        break;
    case 2:
        msg.data [0] = 0x2B;
        msg.data[4]  = data->buf[0];
        msg.data[5]  = data->buf[1];
        break;
    case 3:
        msg.data [0] = 0x23;
        msg.data[4]  = data->buf[0];
        msg.data[5]  = data->buf[1];
        msg.data[6]  = data->buf[2];
        break;
    case 4:
        msg.data [0] = 0x23;
        msg.data[4]  = data->buf[0];
        msg.data[5]  = data->buf[1];
        msg.data[6]  = data->buf[2];
        msg.data[7]  = data->buf[3];
        break;
    default:
        msg.data [0] = 0x21;
        msg.data[4]  = data->len;
        break;
    }
	msg.flags = 0;
	msg.cob   = 0;
	msg.id    = (node & 0x7F) | 0x600;
	msg.length   = 8;
	msg.data[1] = (unsigned char) (index & 0xff);
	msg.data[2] =  (unsigned char)(index >> 8);
	msg.data[3] = (unsigned char) sub;
  canmsg_t rx;
	if (write (fd, &msg, 1))return 3;
  if(read (fd, &rx, 1)) return 2;
  switch (rx.data[0])
  {
    case 0x41: //read
        return read_string(fd,node,&rx,data);
    case 0x61:
        return write_string(fd,node,data);
    default:
        return set_value(&rx,data);
        break;
  }
  return 0;
}

static U32 value_u32(canmsg_t *rx) {
  unsigned value = 0;
  char len = 0;
  if (0x40 != (rx->data[0]&0xE0))
    return 0;
	if (2 & rx->data[0]) {
		// 0: 4, 1: 3, 2: 2, 3: 1
		len   = 4 - (3 & (rx->data[0] >> 2));
		value  = rx -> data[4];
  	if (len > 1) value |= rx->data[5] << 8;
		if (len > 2) value |= rx->data[6] << 16;
		if (len > 3) value |= rx->data[7] << 24;

		// Es wurde eine Zahl der Länge '* size' mit dem Wert '* value' gelesen.
		return value;
	}

	if (0x41 == rx ->data[0]) {
		value = rx -> data[4] | rx->data[5] << 8;
		// Es wurde die Länge einer Zeichenkette gelesen.
		return value;
	}
	return 0;
}

static U32 read_u32(int fd, int node,int index,U8 sub) {
    canmsg_t msg;
    msg.data[0] = 0x40;
    msg.flags   = 0;
    msg.cob     = 0;
    msg.id      = (node & 0x7F) | 0x600;
    msg.length  = 8;
    msg.data[1] = (unsigned char) (index & 0xff);
    msg.data[2] = (unsigned char)(index >> 8);
    msg.data[3] = (unsigned char) sub;
    if(write(can0(), &msg, 1))return 3;
    canmsg_t rx;
    if(read(can0(), &rx, 1)) return 2;
    return value_u32(&rx);
}

static Err write_u32(int fd, int node,int index,U8 sub,U8 len,U32 value) {
    canmsg_t msg;
    msg.data[0] = 0x40;
    msg.flags   = 0;
    msg.cob     = 0;
    msg.id      = (node & 0x7F) | 0x600;
    msg.length  = 8;
    msg.data[1] = (U8) (index & 0xff);
    msg.data[2] = (U8)(index >> 8);
    msg.data[3] = (U8) sub;
    msg.data[4] = (U8)            (unsigned)value;
    msg.data[5] = (U8) (1 < len ? (unsigned)value >>  8 : 0);
    msg.data[6] = (U8) (2 < len ? (unsigned)value >> 16 : 0);
    msg.data[7] = (U8) (3 < len ? (unsigned)value >> 24 : 0);
    if(write(fd, &msg, 1))return 3;
    canmsg_t rx;
    if(read(fd, &rx, 1)) return 2;
    return 0;
}
U32 read_unsigned(int node,int index,unsigned char sub) {
    return read_u32(can0(),node,index,sub);
}
Err write_unsigned(int node,int index,U8 sub,U8 len,U32 value) {
    return write_u32(can0(),node,index,sub,len,value);
}
static char* read_uart(int fd, int node,int index,U8 sub,Data *buffer){
    if (buffer == NULL) {
        static Data local;
        local.len =0;
        memset(local.buf,0,256);
        buffer = &local;
    }
    canmsg_t msg;
    msg.data [0] = 0x40;
    msg.flags = 0;
	msg.cob   = 0;
	msg.id    = (node & 0x7F) | 0x600;
	msg.length   = 8;
	msg.data[1] = (unsigned char) (index & 0xff);
	msg.data[2] =  (unsigned char)(index >> 8);
	msg.data[3] = (unsigned char) sub;
    canmsg_t rx;
	if (write (fd, &msg, 1))return NULL;
    if(read (fd, &rx, 1)) return NULL;
    read_string(fd,node,&rx,buffer);
    return buffer->buf;
}
static Err write_uart(int fd,int node,int index,U8 sub,Data *buffer) {
     if (buffer == NULL) {
        static Data local;
        local.len =0;
        memset(local.buf,0,256);
        buffer = &local;
    }
    canmsg_t       msg;
    msg.data [0] = 0x21;
    msg.flags    = 0;
	msg.cob      = 0;
	msg.id       = (node & 0x7F) | 0x600;
	msg.length   = 8;
	msg.data[1]  = (unsigned char) (index & 0xff);
	msg.data[2]  = (unsigned char)(index >> 8);
	msg.data[3]  = (unsigned char) sub;
    msg.data[4]  = buffer->len;
	if (write (fd, &msg, 1))return 3;
    canmsg_t rx;
    if(read (fd, &rx, 1)) return 2;
    write_string(fd,node,buffer);
    return 0;
}

// Analog node

/// Analog inputs
U32 analog_get_in01(int node) {
    return read_u32(can0(),node,0x6100,0x1);
}
U32 analog_get_in02(int node) {
    return read_u32(can0(),node,0x6100,0x2);
}
U32 analog_get_in03(int node) {
    return read_u32(can0(),node,0x6100,0x3);
}
U32 analog_get_in04(int node) {
    return read_u32(can0(),node,0x6100,0x4);
}
U32 analog_get_in05(int node) {
    return read_u32(can0(),node,0x6110,0x1);
}
// Analog out
U32 analog_get_out(int node) {
    return read_u32(can0(),node,0x6111,0x1);
}
Err analog_set_out(int node,U32 value){
    return write_u32(can0(),node,0x6111,0x1,2,value);
}
// Analog node temperatur
U32 analog_get_temp01(int node) {
    return read_u32(can0(),node,0x6021,0x1);
}
U32 analog_get_temp02(int node) {
    return read_u32(can0(),node,0x6021,0x2);
}
U32 analog_get_temp03(int node) {
    return read_u32(can0(),node,0x6021,0x3);
}

char* analog_get_uart01(int node){
    static Data buffer;
    buffer.len = 0;
    memset(buffer.buf,0,256);
    return read_uart(can0(),node,0x6000,0x1,&buffer);
}
Err analog_set_baut01(int node,U32 baut){
    return write_u32(can0(),node,0x6000,0x4,1,baut);
}
Err analog_set_uart01(int node,char *data){
    static Data buffer;
    buffer.len = strlen(data);
    strncpy(buffer.buf,data,256);
    return write_uart(can0(),node,0x6000,0x1,&buffer);
}
char* analog_get_uart02(int node){
    static Data buffer;
    buffer.len = 0;
    memset(buffer.buf,0,256);
    return read_uart(can0(),node,0x6010,0x1,&buffer);
}
Err analog_set_uart02(int node,char *data){
    static Data buffer;
    buffer.len = strlen(data);
    strncpy(buffer.buf,data,256);
    return write_uart(can0(),node,0x6010,0x1,&buffer);
}
Err analog_set_baut02(int node,unsigned baut){
    return write_u32(can0(),node,0x6010,0x4,2,baut);
}

U32 digital_get_input(int node){
    return read_u32(can0(),node,0x6100,0x1);
}
U32 digital_get_output(int node){
    return read_u32(can0(),node,0x6101,0x1);
}
Err digital_set_output(int node,U32 value){
    return write_u32(can0(),node,0x6101,0x1,2,value);
}

char*    doppelmotor_get_uart01(int node){
    static Data buffer;
    buffer.len = 0;
    memset(buffer.buf,0,256);
    return read_uart(can0(),node,0x6000,0x1,&buffer);
}
char*    doppelmotor_get_uart02(int node){
    static Data buffer;
    buffer.len = 0;
    memset(buffer.buf,0,256);
    return read_uart(can0(),node,0x6010,0x1,&buffer);
}

Err doppelmotor_set_baut01(int node,unsigned baut){
    return write_u32(can0(),node,0x6000,0x4,1,baut);
}
Err doppelmotor_set_baut02(int node,unsigned baut){
    return write_u32(can0(),node,0x6010,0x4,1,baut);
}

Err doppelmotor_set_uart01(int node,char *data){
    static Data buffer;
    buffer.len = strlen(data);
    strncpy(buffer.buf,data,256);
    return write_uart(can0(),node,0x6000,0x1,&buffer);
}
Err doppelmotor_set_uart02(int node,char *data){
    static Data buffer;
    buffer.len = strlen(data);
    strncpy(buffer.buf,data,256);
    return write_uart(can0(),node,0x6010,0x1,&buffer);
}

// Analog extension count outputs
U32 analogext_get_count(U8 out) {
    return read_u32(can0(),0x1C,0x6411,0);
}

// Analog extension read out
U32 analogext_get_out(U8 out) {
    return read_u32(can0(),0x1C,0x6411,out);
}

// Analog extension change out
Err analogext_set_out(U8 out ,U32 value){
    return write_u32(can0(),0x1C,0x6411,out,2,value);
}

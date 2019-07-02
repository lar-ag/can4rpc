#include <fcntl.h>
#include <malloc.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include "can4linux.h"

typedef struct Data{
    unsigned char buf[256];
    unsigned char len;
}Data;


int read_string(int fd,int node, canmsg_t *rx, Data *data) {
    unsigned char len = rx->data[4];
    data->len = 0;
    canmsg_t msg;
    msg.flags = 0;
	msg.cob   = 0;
	msg.id    = (node & 0x7F) | 0x600;
    msg.data[0] =  0x60;

    while (len <= data->len)
    {

       	if(write (fd, &msg, 1)) return 1;
        canmsg_t nrx;
        if(read (fd, &nrx, 1)) return 1;
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
int write_string(int fd,int node, Data *data) {
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
       	if(write (fd, &msg, 1)) return 1;
        if(read (fd, &nrx, 1)) return 1;
        toggle ^= 0x10;
        len -=l;
    }
    return 0;
}



int set_value(canmsg_t *rx, Data *data) {
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
int can_msg (int fd, int node, int index, unsigned char sub, Data data)
{

	canmsg_t msg;
    switch (data.len)
    {
    case 0:
        msg.data [0] = 0x40;
        break;
    case 1:
        msg.data [0] = 0x2F;
        msg.data[4]  = data.buf[0];
        break;
    case 2:
        msg.data [0] = 0x2B;
        msg.data[4]  = data.buf[0];
        msg.data[5]  = data.buf[1];
        break;
    case 3:
        msg.data [0] = 0x23;
        msg.data[4]  = data.buf[0];
        msg.data[5]  = data.buf[1];
        msg.data[6]  = data.buf[2];
        break;
    case 4:
        msg.data [0] = 0x23;
        msg.data[4]  = data.buf[0];
        msg.data[5]  = data.buf[1];
        msg.data[6]  = data.buf[2];
        msg.data[7]  = data.buf[3];
        break;
    default:
        msg.data [0] = 0x21;
        msg.data[4]  = data.len;
        break;
    }
	msg.flags = 0;
	msg.cob   = 0;
	msg.id    = (node & 0x7F) | 0x600;
	msg.length   = 8;
	msg.data[1] = (unsigned char) (index & 0xff);
	msg.data[2] =  (unsigned char)(index >> 8);
	msg.data[3] = (unsigned char) sub;
	int res = write (fd, &msg, 1);
    if (res != 0) return res;
    canmsg_t rx;
    if(read (fd, &rx, 1)) return 1;
    switch (rx.data[0])
    {
    case 0x41: //read
        return read_string(fd,node,&rx,&data);
    case 0x61:
        return write_string(fd,node,&data);
    default:
        return set_value(&rx,&data);
        break;
    }
    return 0;
}

int read_ain() {
    return 3;
}
// int main(int argc, char *argv[]) {

// }//

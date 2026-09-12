// The SIXTEEN_3 and weighted readings used by combo2. This is a complete
// word pass: TANCH is inert here, and IFIX freezes every operator but IMSCRIB
// and IFIX. It is not the classical stack machine's instruction-pointer walk.
__device__ void banked_word(const unsigned char *word,unsigned int len,
                            unsigned int *out) {
    unsigned int weights[4]={0,0,0,0};
    unsigned int frames[PROGRAM_CAP][4];
    unsigned int depth=0,landing=0,live=0,deposits=0,inert=0;
    unsigned int exposed=0,exposed_units=0,cleared=0,restored=0,seeded=0;
    bool fixed=false,nonempty=false;
    for(unsigned int ip=0;ip<len;ip++) {
        unsigned char tok=word[ip];
        // Extended spellings have the same tri face. ROTAT is an operator
        // on the word, outside this twelve-mark pass.
        if(tok==15) continue;
        if(tok==12) tok=6;
        if(tok==13) tok=7;
        if(tok==14) tok=10;
        if(fixed && tok!=11 && tok!=8) { ++inert; continue; }
        if(tok==6) {
            for(int j=0;j<4;j++) frames[depth][j]=0;
            ++depth;
        } else if(tok==7) {
            if(depth) {
                --depth;
                nonempty=false;
                for(int j=0;j<4;j++) {
                    unsigned int value=frames[depth][j];
                    if(value) landing|=1u<<j;
                    if(value>weights[j]) { restored+=value-weights[j]; weights[j]=value; }
                    if(depth && value>frames[depth-1][j]) frames[depth-1][j]=value;
                    if(weights[j]) nonempty=true;
                }
            }
        } else if(tok==0 || tok==3) {
            unsigned int lost=0,banked=0;
            for(int j=0;j<4;j++) {lost+=weights[j];weights[j]=0;}
            for(unsigned int d=0;d<depth;d++)
                for(int j=0;j<4;j++) banked+=frames[d][j];
            if(lost) {
                ++live;
                if(!banked) {++exposed;exposed_units+=lost;}
            }
            cleared+=lost; landing=0; nonempty=false;
            // VINIT resets the whole process; AREV leaves the frames open.
            if(tok==0) depth=0;
        } else if(tok==2 || tok==8) {
            if(!landing) landing=1;
            if(!nonempty) {nonempty=true;++seeded;}
        } else if(tok==11) {
            fixed=true;
        } else {
            unsigned int touched=tok==5?1:tok==9?2:tok==10?12:0;
            if(touched) {++deposits;nonempty=true;}
            landing|=touched;
            for(int j=0;j<4;j++) if(touched&(1u<<j)) {
                ++weights[j];
                if(depth) ++frames[depth-1][j];
            }
        }
    }
    for(int j=0;j<4;j++) out[j]=weights[j];
    out[4]=landing; out[5]=live; out[6]=deposits; out[7]=inert;
    out[8]=exposed; out[9]=exposed_units; out[10]=cleared;
    out[11]=restored; out[12]=seeded; out[13]=depth; out[14]=fixed;
    out[15]=len;
}

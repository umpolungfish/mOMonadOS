// Exact joint high-part contraction inside the phase process. Width <= 30
// keeps the signed floor-sum intermediates within 64 bits.
typedef unsigned long long JU;
typedef long long JS;
__device__ JS jdiv(JS a, JS m) { JS q=a/m, r=a%m; return q-(r<0); }
__device__ JS jmod(JS a, JS m) { JS r=a%m; return r<0?r+m:r; }
__device__ JS jfloor(JS n, JS m, JS a, JS b) {
    JS total=0;
    for (;;) {
        JS q=jdiv(a,m); a-=q*m; total+=q*(n*(n-1)/2);
        q=jdiv(b,m); b-=q*m; total+=q*n;
        JS top=a*n+b;
        if (top<m) return total;
        n=top/m; b=top%m; JS old=m; m=a; a=old;
    }
}
__device__ JU jinv(JU a, JU mask) {
    JU x=1; for(int i=0;i<6;i++) x=(x*(2-a*x))&mask; return x;
}
struct JNode { JU p,q; unsigned int t; };
__device__ JNode jnode(JU p,JU q,unsigned int t) {
    JNode n; n.p=p; n.q=q; n.t=t; return n;
}
struct JBox { JU pl,ph,ql,qh; };
__device__ JU jmax(JU a,JU b) {return a>b?a:b;}
__device__ JU jmin(JU a,JU b) {return a<b?a:b;}
__device__ bool jbounds(JU n,JNode node,unsigned int m,JBox &b) {
    JU s=1ULL<<node.t, mask=s-1;
    b.pl=b.ql=1ULL<<(m-1); b.ph=b.qh=(1ULL<<m)-1;
    for(int round=0;round<4;round++) {
        if(b.ph<node.p || b.qh<node.q) return false;
        b.pl+=(node.p-b.pl)&mask; b.ph-=(b.ph-node.p)&mask;
        b.ql+=(node.q-b.ql)&mask; b.qh-=(b.qh-node.q)&mask;
        if(b.pl>b.ph || b.ql>b.qh || b.pl*b.ql>n || b.ph*b.qh<n) return false;
        if(round==3) return true;
        b.pl=jmax(b.pl,(n+b.qh-1)/b.qh);
        b.ph=jmin(jmin(b.ph,n/b.ql),b.qh);
        if(b.pl>b.ph || !b.ph) return false;
        b.ql=jmax(jmax(b.ql,(n+b.ph-1)/b.ph),b.pl);
        b.qh=jmin(b.qh,n/b.pl);
    }
    return true;
}
__device__ JS jcount(JS left,JS right,JS s,JS slope,JS offset,JS bl,JS bh) {
    JS shift=offset+slope*left, length=right-left+1;
    return jfloor(length,s,-slope,bh-shift)-jfloor(length,s,-slope,bl-1-shift);
}
// Exact rational envelopes of the full product relation. Signed 128-bit
// coefficients avoid rounding the tangent or secant across an integer tuple.
typedef __int128 JW;
__device__ JW jwdiv(JW a,JW m) { JW q=a/m,r=a%m; return q-(r<0); }
__device__ JW jwfloor(JW n,JW m,JW a,JW b) {
    JW total=0;
    for(;;) {
        JW q=jwdiv(a,m); a-=q*m; total+=q*(n*(n-1)/2);
        q=jwdiv(b,m); b-=q*m; total+=q*n;
        JW top=a*n+b;
        if(top<m) return total;
        n=top/m; b=top%m; JW old=m; m=a; a=old;
    }
}
struct JStrip { JW um,ua,ub,lm,la,lb; };
__device__ JStrip jstrip(JU n,JNode node,JU pl,JU ph) {
    JW s=(JU)1<<node.t, length=(ph-pl)/(JU)s+1;
    JS inv=(JS)jinv(node.p,(JU)s-1);
    JS slope=jmod(-(JS)node.q*inv,(JS)s);
    JS h=((JS)n-(JS)(node.p*node.q))/(JS)s;
    JS offset=jmod(h*inv,(JS)s);
    JW a0=(pl-node.p)/(JU)s;
    JW qb=node.q+s*(offset+slope*a0);
    JW center=pl+s*(length/2),ud=(JW)pl*ph,ld=center*center;
    JStrip z;
    z.um=ud*s*s; z.ua=-(JW)n*s-ud*s*slope; z.ub=(JW)n*ph-ud*qb;
    z.lm=ld*s*s; z.la=-(JW)n*s-ld*s*slope; z.lb=(JW)n*(2*center-pl)-ld*qb;
    return z;
}
__device__ JW jstrip_count(JStrip z,JU l,JU r) {
    JW size=r-l+1;
    return jwfloor(size,z.um,z.ua,z.ub+z.ua*l)
         + jwfloor(size,z.lm,-z.la,-z.lb-z.la*l)+size;
}
// Return false when the envelope is still dense. A sparse envelope is
// unranked directly; its tuples are checked against the exact product.
__device__ bool jcurve(JU n,JNode node,JBox box,JU &factor,JU &products) {
    JU s=1ULL<<node.t,length=(box.ph-box.pl)/s+1;
    JU pieces=jmin(length,16),total=0;
    for(JU part=0;part<pieces;part++) {
        JU first=length*part/pieces,end=length*(part+1)/pieces;
        JStrip z=jstrip(n,node,box.pl+s*first,box.pl+s*(end-1));
        JW count=jstrip_count(z,0,end-first-1);
        if(count>8-total) return false;
        total+=(JU)count;
    }
    if(!total) return true;
    for(JU part=0;part<pieces;part++) {
        JU first=length*part/pieces,end=length*(part+1)/pieces;
        JU start=box.pl+s*first;
        JStrip z=jstrip(n,node,start,box.pl+s*(end-1));
        JU amount=(JU)jstrip_count(z,0,end-first-1);
        for(JU rank=0;rank<amount;rank++) {
            JU l=0,r=end-first-1,k=rank;
            while(l<r) {
                JU mid=l+(r-l)/2;
                JU count=(JU)jstrip_count(z,l,mid);
                if(k<count) r=mid; else {l=mid+1;k-=count;}
            }
            JU p=start+s*l,q=n/p;
            ++products;
            if(p>1 && p<=q && q>=box.ql && q<=box.qh &&
               (q&(s-1))==node.q && p*q==n) factor=jmin(factor,p);
        }
    }
    return true;
}
__device__ void joint_process(JU n,unsigned int m,JU *out,
                              const unsigned char *word,unsigned int len) {
    unsigned int root=m<JOINT_ROOT_BITS?m:JOINT_ROOT_BITS;
    JU gid=(JU)blockIdx.x*blockDim.x+threadIdx.x;
    if(gid>=(1ULL<<(root-1))) return;
    JNode pending[32]; int top=1;
    JU p0=2*gid+1, mask=(1ULL<<root)-1;
    pending[0]=jnode(p0,(n*jinv(p0,mask))&mask,root);
    JU visits=0, products=0, curves=0, lifts=0;
    while(top) {
        JNode node=pending[--top]; JBox box;
        bool live=true,dense=false; JU factor=n;
        JS al=0,ah=0,bl=0,bh=0,amount=0,slope=0,offset=0;
        JU s=1ULL<<node.t;
        for(unsigned int ip=0;ip<len;ip++) {
            switch(word[ip]) {
                case 0: ++visits; break;
                case 8: break;
                case 6: live=jbounds(n,node,m,box); break;
                case 2:
                    if(live) {al=(box.pl-node.p)/s; ah=(box.ph-node.p)/s;}
                    break;
                case 5: live=live && al<=ah; break;
                case 3:
                    if(live) {bl=(box.ql-node.q)/s; bh=(box.qh-node.q)/s;}
                    break;
                case 9: live=live && bl<=bh; break;
                case 4:
                    if(live) {
                        JS na=ah-al+1, nb=bh-bl+1;
                        dense=na*(nb/(JS)s)>8 || nb*(na/(JS)s)>8;
                        if(!dense) {
                            JS inv=(JS)jinv(node.p,s-1);
                            JS h=((JS)n-(JS)(node.p*node.q))/(JS)s;
                            slope=jmod(-(JS)node.q*inv,(JS)s);
                            offset=jmod(h*inv,(JS)s);
                            amount=jcount(al,ah,s,slope,offset,bl,bh);
                            dense=amount>8;
                        }
                    }
                    break;
                case 10: break; // retain the joint relation until its fuse
                case 7:
                    if(live && !dense) {
                        for(JS rank=0;rank<amount;rank++) {
                            JS l=al,r=ah,k=rank;
                            while(l<r) {
                                JS mid=l+(r-l)/2;
                                JS count=jcount(l,mid,s,slope,offset,bl,bh);
                                if(k<count) r=mid; else {l=mid+1;k-=count;}
                            }
                            JS b=bl+jmod(offset+slope*l-bl,s)+k*(JS)s;
                            JU p=node.p+s*(JU)l,q=node.q+s*(JU)b;
                            ++products;
                            if(p>1 && p<=q && p*q==n) factor=jmin(factor,p);
                        }
                    } else if(live) {
                        ++curves;
                        if(jcurve(n,node,box,factor,products) || node.t>=m) break;
                        // Lift the low-part relation once. The complement bit
                        // is forced by the current residual and chosen arm.
                        JU carry=((n-node.p*node.q)>>node.t)&1;
                        ++lifts;
                        pending[top++]=jnode(node.p,node.q+carry*s,node.t+1);
                        pending[top++]=jnode(node.p+s,node.q+(carry^1)*s,node.t+1);
                    }
                    break;
                case 11: if(factor<n) atomicMin(out,factor); break;
                case 1: ip=len; break;
            }
        }
    }
    atomicAdd(out+1,visits); atomicAdd(out+2,products);
    atomicAdd(out+3,curves); atomicAdd(out+4,lifts);
}

for f in *
    do
        if [ -d ${f} ]
            then :
        elif [ ${f:0:3} = 'ABC' ] || [ ${f:0:3} = 'ARC' ] || [ ${f:0:3} = 'AGC' ] || [ ${f:0:3} = 'ATC' ] 
            then mv ${f} ./${f:0:3}/${f:0:3}-${f:6:1}/ || mkdir ${f:0:3}; mv ${f} ./${f:0:3}/${f:0:3}-${f:6:1}/ || mkdir ./${f:0:3}/${f:0:3}-${f:6:1}/; mv ${f} ./${f:0:3}/${f:0:3}-${f:6:1}/
        elif [ ${f:0:3} = 'AOJ' ] || [ ${f:0:3} = 'JOI' ]
            then mv ${f} ./${f:0:3}/ || mkdir ${f:0:3}; mv ${f} ./${f:0:3}/
        elif [ ${f:0:4} = 'EDPC' ]
            then mv ${f} ./${f:0:4}/ || mkdir ${f:0:4}; mv ${f} ./${f:0:4}/
        elif [ ${f} != "organize.sh" ] && [ ${f} != "README.md" ] && [ ${f} != "easy.sh" ]
            then mv ${f} ./Others/ || mkdir Others; mv ${f} ./Others/
        fi
    done
exit 0